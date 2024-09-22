mod config;
mod notes;
mod cache;
mod templates;
mod tools;
mod types;
mod highlighter;

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger::{self, Env};
use std::process;
use log::info;

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

    if !tools::is_dir(&config.rusty_dir_templates) {
        println!("ERROR: directory `{}` does not exist", config.rusty_dir_templates);
        process::exit(1);
    }
    if !tools::is_dir(&config.rusty_dir_notes) {
        info!("WARN: directory `{}` does not exist", config.rusty_dir_notes);
    }
    info!("notes root directory: {}", config.rusty_dir_notes);

    let notes_prefix = match config.rusty_url_prefix.trim_matches('/') {
        "" => "/".to_string(),
        s => format!("/{}/", s),
    };
    info!("notes_prefix: {}", notes_prefix);
    info!("Rusty Notes running at: http://{}{}", config.rusty_server_addr, notes_prefix);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::PayloadConfig::new(1024 * 1024 * 2))
            .app_data(web::FormConfig::default().limit(1024 * 1024 * 2))

            .wrap(Logger::default())

            .route(&format!("{}", notes_prefix), web::get().to(notes::web::home))
            .route(&format!("{}edit/{{tail:.*}}", notes_prefix), web::get().to(notes::web::edit_note_get))
            .route(&format!("{}edit/{{tail:.*}}", notes_prefix), web::post().to(notes::web::edit_note_post))

            .route("/stc/{tail:.*}", web::get().to(notes::web::serve_statics))
            .route("/code/{tail:.*}", web::get().to(notes::web::serve_code))
            // need to be the last one for notes-prefix: "/"
            .route(&format!("{}{{tail:.*}}", notes_prefix), web::get().to(notes::web::note_detail))

            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind(config.rusty_server_addr.clone())?
    .workers(2)
    .run();
    server.await
}
