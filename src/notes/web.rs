use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use actix_files::NamedFile;
use actix_session::Session;
use actix_web::http::header::HeaderValue;
use actix_web::http::header::ContentDisposition;
use actix_web::{web, HttpResponse, Result, Responder};
use serde::Deserialize;
use tera::Context;

use crate::config::Config;
use crate::templates::TEMPLATES;
use crate::notes::utils::fetch_all_notes;
use crate::notes::utils::reset_note_title_cache;
use crate::notes::utils::note_path_to_items;
use crate::notes::utils::get_notes_by_search;
use crate::notes::utils::render_doc;
use crate::types::NoteItem;


#[derive(Deserialize)]
pub struct Info {
    category: Option<String>,
    q: Option<String>,
}

pub async fn home(info: web::Query<Info>, _session: Session)
    -> Result<HttpResponse>
{
    let dir_notes = get_notes_dir();
    let dir_notes = dir_notes.to_string_lossy().to_owned();
    let category = if let Some(x) = &info.category { x.clone() } else { String::new() };

    let mut q = if let Some(x) = &info.q { x.clone() } else { String::new() };
    let mut files_selected: Vec<String> = Vec::new();
    if !q.is_empty() {
        let _info = get_notes_by_search(&q, &dir_notes, &mut files_selected);
        if !_info.is_empty() {
            q = _info;
        }
    }

    let mut records: Vec<NoteItem> = Vec::new();
    if !files_selected.is_empty() || !q.is_empty() {
        records = note_path_to_items(files_selected, &dir_notes);
    } else {
        let limit = if !q.is_empty() || !category.is_empty() { 0 } else { 27 };
        let category_dir = format!("{}/", category);
        for item in fetch_all_notes(&dir_notes, &dir_notes, limit) {
            if !category.is_empty() && category != "_all" && !(
                    item.root_dir == category || item.root_dir.starts_with(&category_dir)) {
                continue;
            }
            records.push(item)
        }
    }

    let mut context = Context::new();
    context.insert("in_home", &true);
    context.insert("records", &records);
    context.insert("category", &category);
    context.insert("q", &q);

    render("notes/home.html", &context)
}

pub async fn serve_statics(wpath: web::Path<String>) -> Result<NamedFile> {
    let dir_notes = get_notes_dir();
    let source_file = dir_notes.join("static").join(wpath.to_string());
    Ok(NamedFile::open(&source_file)?)
}

pub async fn serve_code(path: web::Path<String>) -> Result<NamedFile> {
    let config = Config::from_env().unwrap();
    let source_file = format!("{}/static/code/{}", config.dir_notes, path);
    let val = HeaderValue::from_static("inline");
    let cd: ContentDisposition = ContentDisposition::from_raw(&val)?;

    if Path::new(&source_file).exists() {
        let f = NamedFile::open(&source_file)?
            .set_content_disposition(cd)
            .prefer_utf8(true)
            .set_content_type(mime::TEXT_PLAIN);

        Ok(f)
    } else {
        Err(actix_web::error::ErrorNotFound("File not found."))
    }
}

pub async fn note_detail(path: web::Path<String>, _session: Session) -> Result<HttpResponse> {
    let dir_notes = get_notes_dir();
    let doc_ = dir_notes.join(path.to_string());
    let doc_ = doc_.to_string_lossy().to_owned();

    let mut context = Context::new();
    let (html, title) = render_doc(&doc_);
    context.insert("content", &html);
    let edit_url = format!("/notes/edit/{}", path);
    context.insert("edit_url", &edit_url);
    context.insert("title", &title);

    render("notes/detail.html", &context)
}

pub async fn edit_note_get(path: web::Path<String>, _session: Session) -> impl Responder {
    let dir_notes = get_notes_dir();
    let md_file = dir_notes.join(path.to_string());
    let md_file = md_file.to_string_lossy().into_owned();
    reset_note_title_cache(&md_file);

    let content = fs::read_to_string(&md_file)
        .expect("Something went wrong reading the file");

    let mut context = Context::new();
    context.insert("content", &content);
    let path: String = path.into_inner();
    context.insert("note_path", &path);
    match TEMPLATES.render("notes/edit.html", &context) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => {
            let err_info = format!("error: {:?}", e);
            HttpResponse::Ok().body(err_info)
        }
    }
}

#[derive(Deserialize)]
pub struct EditNote {
    note: String,
}

pub async fn edit_note_post(
    form: web::Form<EditNote>, path: web::Path<String>, _session: Session,
) -> impl Responder
{
    let dir_notes = get_notes_dir();
    let md_file = dir_notes.join(path.to_string());
    let md_file = md_file.to_string_lossy().into_owned();
    reset_note_title_cache(&md_file);

    let mut f = File::create(&md_file).expect("open file error");
    let data = form.note.replace("\r", "");
    let data = data.trim_end();
    f.write_all(data.as_bytes()).expect("write_all error");
    f.write_all(b"\n").expect("write_all error");
    f.sync_all().expect("sync_all error");

    let note_url = format!("/notes/{}", path);
    HttpResponse::Found().append_header(("Location", note_url)).finish()
}

fn get_notes_dir() -> PathBuf {
    let config = Config::from_env().unwrap();
    Path::new(&config.dir_notes).to_path_buf()
}

fn render(template: &str, context: &Context) -> Result<HttpResponse, actix_web::Error> {
    match TEMPLATES.render(template, context) {
        Ok(html) => {
            Ok(HttpResponse::Ok().body(html))
        }
        Err(e) => {
            let msg = format!("error: {:?}", e);
            Ok(HttpResponse::NotFound().body(msg))
        }
    }
}
