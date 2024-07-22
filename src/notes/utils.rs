use comrak::{markdown_to_html, ComrakOptions};
use log::error;
use regex::Regex;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::time::UNIX_EPOCH;
use walkdir::WalkDir;

use crate::cache::{cache_delete, cache_get, cache_set_1h};
use crate::highlighter::highlight_html;
use crate::types::NoteItem;

fn title_string<R>(mut rdr: R) -> String
    where R: BufRead,
{
    let mut first_line = String::new();

    match rdr.read_line(&mut first_line) {
        Ok(_) => {}
        Err(_) => {
            return String::new();
        }
    }

    // Where do the leading hashes stop?
    let last_hash = first_line
        .char_indices()
        .skip_while(|&(_, c)| c == '#')
        .next()
        .map_or(0, |(idx, _)| idx);

    // Trim the leading hashes and any whitespace
    first_line[last_hash..].trim().into()
}

fn walk_dir(root_dir: &str, suffix: &str) -> Vec<String> {
    let mut result = Vec::new();
    for entry in WalkDir::new(root_dir) {
        if entry.is_err() {
            return result;
        }
        // info!("{:?}", entry);

        let path = entry.unwrap().into_path();
        let _file_path = format!("{}", path.to_string_lossy());
        if !_file_path.ends_with(suffix) {
            continue;
        }

        // skip soft links
        if !Path::new(&_file_path).exists() {
            continue;
        }

        // skip docks in subusers/
        match path.into_os_string().into_string() {
            Ok(x) => {
                if x.contains("/subusers/") {
                    continue;
                }
            }
            Err(_) => {}
        }

        result.push(_file_path.to_string())
    }
    result
}

pub fn reset_note_title_cache(file_path: &str) {
    let note = NoteItem::new(file_path);
    let key_ = note.get_cache_key_for_title();
    cache_delete(&key_);
}

pub fn get_note_title(note: &NoteItem) -> String {
    let key_ = note.get_cache_key_for_title();
    if let Some(title) = cache_get(&key_) {
        return title;
    }

    let buffer = BufReader::new(fs::File::open(&note.file_path).unwrap());
    let title = title_string(buffer);
    cache_set_1h(&key_, &title);
    title
}

pub fn note_path_to_items(path_list: Vec<String>, strip_prefix: &str) -> Vec<NoteItem> {
    let mut note_list = Vec::new();
    for file_path in path_list {
        let mut note_item = NoteItem::new(&file_path);
        let metadata;
        match fs::metadata(&file_path) {
            Ok(x) => {
                metadata = x;
            }
            Err(e) => {
                error!("metadata error: {:?} {}", e, &file_path);
                continue;
            }
        }
        let modified_ts: i64 = if let Ok(time) = metadata.modified() {
            time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
        } else {
            0
        };
        note_item.set_modified(modified_ts);
        note_list.push(note_item);
    }

    for note_item in note_list.as_mut_slice() {
        let title = get_note_title(&note_item);

        let path = Path::new(&note_item.file_path);
        let parent = path.parent();
        let file_name = match path.file_name() {
            Some(s) => {
                s.to_string_lossy().into_owned()
            }
            None => {
                "no-name".to_string()
            }
        };

        if !parent.is_none() {
            let root_dir = parent.unwrap().to_str().unwrap();
            let root_dir = root_dir.replacen(strip_prefix.trim_matches('/'), "", 1);
            let root_dir = root_dir.trim_start_matches('/');
            note_item.set_root_dir(&root_dir);
        }
        note_item.set_file_name(&file_name);
        note_item.set_title(&title);
    }

    note_list
}

pub fn fetch_all_notes(dir_: &str, strip_prefix: &str, limit: usize) -> Vec<NoteItem> {
    let mut note_list = Vec::new();

    for file_path in walk_dir(dir_, ".md") {
        let mut note_item = NoteItem::new(&file_path);
        let metadata = fs::metadata(&file_path).unwrap();
        let modified_ts: i64 = if let Ok(time) = metadata.modified() {
            time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
        } else {
            0
        };
        note_item.set_modified(modified_ts);
        note_list.push(note_item);
    }

    note_list.sort_by(|a, b| b.modified_ts.cmp(&a.modified_ts));
    if limit > 0 {
        note_list.truncate(limit);
    }

    for note_item in note_list.as_mut_slice() {
        let title = get_note_title(&note_item);

        let path = Path::new(&note_item.file_path);
        let parent = path.parent();
        let file_name = match path.file_name() {
            Some(s) => {
                s.to_string_lossy().into_owned()
            }
            None => {
                "no-name".to_string()
            }
        };

        if !parent.is_none() {
            let root_dir = parent.unwrap().to_str().unwrap();
            let root_dir = root_dir.replacen(strip_prefix.trim_matches('/'), "", 1);
            let root_dir = root_dir.trim_start_matches('/');
            note_item.set_root_dir(&root_dir);
        }
        note_item.set_file_name(&file_name);
        note_item.set_title(&title);
    }

    note_list
}

pub fn expand_collapse_tags(html: &str) -> String {
    let re = Regex::new(r#"<collapse title="(?P<title>[^"]+)">"#).unwrap();
    let to_ = r#"<div class="collap"> <div class="collap-header"> <summary> <b>${title}</b> </summary> </div> <div class="collap-content" style="display: none;">"#;
    let html_new = re.replace_all(html, to_);

    let re = Regex::new(r"</collapse>").unwrap();
    let html_new = re.replace_all(&html_new, "</div></div>");

    html_new.to_string()
}

fn create_new_note(q: &str, notes_dir: &str) -> String {
    let v: Vec<&str> = q.split(':').collect();

    let md_path: &str;
    match v.last() {
        Some(x) => {
            md_path = x;
        }
        None => {
            return "USE touch::foobar.md".to_string();
        }
    }

    let mut v2: Vec<&str> = md_path.split('/').collect();
    if v2.len() == 1 {
        v2.insert(0, "notes");
    } else if v2.len() > 2 {
        return "only one level supported".to_string();
    }

    let md_name = if v2[1].ends_with(".md") {
        v2[1].to_string()
    } else {
        format!("{}.md", v2[1])
    };

    fs::create_dir_all(notes_dir).ok();

    let path = Path::new(notes_dir);
    let md_file = path.join(v2[0]).join(md_name);
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(md_file)
        .expect("Unable to open file");

    f.write_all(b"newly-created\n").ok();

    "newly-created".to_string()
}

pub fn get_notes_by_search(q: &str, notes_dir: &str, note_list: &mut Vec<String>) -> String {
    if q.starts_with("touch::") {
        return create_new_note(q, notes_dir);
    }

    crate::notes::search::search_notes_by_fd(q, notes_dir, note_list);
    crate::notes::search::search_notes_by_rg(q, notes_dir, note_list);

    String::new()
}

pub fn render_text(txt: &str) -> String {
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;
    options.extension.table = true;
    options.extension.header_ids = Some("user-content-".to_string());
    let html = markdown_to_html(txt, &options);
    html
}

pub fn render_doc(mk_file: &str) -> (String, String) {
    if !Path::new(mk_file).exists() {
        return ("404 - no such note".to_string(), String::new());
    }

    let txt = fs::read_to_string(mk_file)
        .expect("Something went wrong reading the file");

    // convert markdown text to html text with lib comrak.
    let html = render_text(&txt);
    let html = highlight_html(&html);
    let html = expand_collapse_tags(&html);
    let note = NoteItem::new(mk_file);
    let title = get_note_title(&note);

    (html, title)
}
