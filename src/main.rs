mod config;
mod notes;
mod cache;
mod templates;
mod tools;
mod types;
mod highlighter;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::cookie::Key;
use actix_web::middleware::{self, Logger};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_session::config::PersistentSession;
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

    if !tools::is_dir(&config.dir_templates) {
        println!("ERROR: directory `{}` does not exist", config.dir_templates);
        process::exit(1);
    }
    if !tools::is_dir(&config.dir_notes) {
        info!("WARN: directory `{}` does not exist", config.dir_notes);
    }

    info!("serving directory: {}", config.dir_notes);
    let cookie_key = tools::get_cookie_key()?;
    let cookie_key = Key::from(cookie_key.as_bytes());
    info!("Rusty Notes running at: http://{}/notes/", config.server_addr);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::PayloadConfig::new(1024 * 1024 * 2))
            .app_data(
                web::FormConfig::default().limit(1024 * 1024 * 2)
            )

            .wrap(Logger::default())
            .wrap(middleware::DefaultHeaders::new().add(("built-with", "mind")))
            // Add session management
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    cookie_key.clone()
                ).session_lifecycle(
                    PersistentSession::default().session_ttl(time::Duration::days(90))
                ).build()
            )

            .route("/", web::get().to(index))

            .route("/notes/", web::get().to(notes::web::home))
            .route("/notes/edit/{tail:.*}", web::get().to(notes::web::edit_note_get))
            .route("/notes/edit/{tail:.*}", web::post().to(notes::web::edit_note_post))
            .route("/notes/{tail:.*}", web::get().to(notes::web::note_detail))

            // public URIs
            .route("/stc/{tail:.*}", web::get().to(notes::web::serve_statics))
            .route("/code/{tail:.*}", web::get().to(notes::web::serve_code))

            .default_service(
                web::to(|| HttpResponse::NotFound())
            )
    })
    .bind(config.server_addr.clone())?
    .workers(4)
    .run();
    server.await
}

async fn index(_session: Session) -> impl Responder {
    let html = format!("Hello {}", "世界！");
    HttpResponse::Ok().body(html)
}
