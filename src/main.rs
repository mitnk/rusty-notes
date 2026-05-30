mod cache;
mod config;
mod highlighter;
mod notes;
mod templates;
mod tools;
mod types;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use env_logger::{self, Env};
use log::info;
use std::process;

// Bundled into the binary at build time. Seeded into the notes directory when
// it starts out empty so first-time users discover how to put rusty-notes
// behind an authenticating reverse proxy.
const NGINX_DOC: &str = include_str!("../docs/put-rusty-notes-behind-nginx.md");

// Returns true if `notes_dir` has no notes yet: no `.md` files and no
// non-hidden subdirectories.
fn notes_dir_is_empty(notes_dir: &str) -> bool {
    let entries = match std::fs::read_dir(notes_dir) {
        Ok(x) => x,
        Err(_) => return false,
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with('.') {
            continue;
        }
        match entry.file_type() {
            Ok(ft) if ft.is_dir() => return false,
            Ok(_) if name.ends_with(".md") => return false,
            _ => {}
        }
    }
    true
}

// When the notes directory is empty, drop a copy of the nginx doc at
// `rusty-notes/put-rusty-notes-behind-nginx.md` so it shows up in the UI.
fn seed_nginx_doc(notes_dir: &str) {
    let dir = std::path::Path::new(notes_dir).join("rusty-notes");
    if let Err(e) = std::fs::create_dir_all(&dir) {
        info!("failed to create seed doc directory: {}", e);
        return;
    }
    let file_path = dir.join("put-rusty-notes-behind-nginx.md");
    match std::fs::write(&file_path, NGINX_DOC) {
        Ok(_) => info!(
            "notes directory was empty; seeded {}",
            file_path.to_string_lossy()
        ),
        Err(e) => info!("failed to write seed doc: {}", e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    dotenv().ok();
    let config = match config::Config::from_env() {
        Ok(x) => x,
        Err(e) => {
            println!("config from_env error: {}", e);
            process::exit(1);
        }
    };

    if !tools::is_dir(&config.rusty_notes_dir) {
        match std::fs::create_dir_all(&config.rusty_notes_dir) {
            Ok(_) => info!("Created notes directory: {}", config.rusty_notes_dir),
            Err(e) => {
                println!(
                    "ERROR: failed to create notes directory `{}`: {}",
                    config.rusty_notes_dir, e
                );
                process::exit(1);
            }
        }
    }
    info!("notes root directory: {}", config.rusty_notes_dir);

    if notes_dir_is_empty(&config.rusty_notes_dir) {
        seed_nginx_doc(&config.rusty_notes_dir);
    }

    let notes_prefix = match config.rusty_url_prefix.trim_matches('/') {
        "" => "/".to_string(),
        s => format!("/{}/", s),
    };
    info!(
        "Rusty Notes running at: http://{}{}",
        config.rusty_server_addr, notes_prefix
    );

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::PayloadConfig::new(1024 * 1024 * 2))
            .app_data(web::FormConfig::default().limit(1024 * 1024 * 2))
            .wrap(Logger::default())
            .route(&notes_prefix.to_string(), web::get().to(notes::web::home))
            .route(
                &format!("{}create/", notes_prefix),
                web::get().to(notes::web::create_note_get),
            )
            .route(
                &format!("{}create/", notes_prefix),
                web::post().to(notes::web::create_note_post),
            )
            .route(
                &format!("{}edit/{{tail:.*}}", notes_prefix),
                web::get().to(notes::web::edit_note_get),
            )
            .route(
                &format!("{}edit/{{tail:.*}}", notes_prefix),
                web::post().to(notes::web::edit_note_post),
            )
            .route("/stc/{tail:.*}", web::get().to(notes::web::serve_statics))
            .route("/code/{tail:.*}", web::get().to(notes::web::serve_code))
            // need to be the last one for notes-prefix: "/"
            .route(
                &format!("{}{{tail:.*}}", notes_prefix),
                web::get().to(notes::web::note_detail),
            )
            .default_service(web::to(HttpResponse::NotFound))
    })
    .bind(config.rusty_server_addr.clone())?
    .workers(2)
    .run();
    server.await
}
