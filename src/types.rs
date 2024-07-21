use serde::Serialize;
use time::{OffsetDateTime, UtcOffset, format_description};

#[derive(Debug, Serialize)]
pub struct NoteItem {
    pub file_path: String,
    pub root_dir: String,
    pub title: String,
    pub file_name: String,
    pub modified_ts: i64,
    pub modified_str: String,
}

impl NoteItem {
    pub fn new(file_path: &str) -> NoteItem {
        NoteItem {
            file_path: file_path.to_string(),
            root_dir: String::new(),
            title: String::new(),
            file_name: String::new(),
            modified_ts: 0,
            modified_str: String::new(),
        }
    }

    pub fn set_modified(&mut self, modified_ts: i64) {
        let dt = OffsetDateTime::from_unix_timestamp(modified_ts)
            .expect("valid timestamp")
            .to_offset(
                UtcOffset::from_whole_seconds(8 * 3600)
                .expect("valid offset")
            );

        // Custom format for "YYYY-mm-dd HH:MM"
        let format = format_description::parse(
            "[year]-[month]-[day] [hour repr:24]:[minute]"
        ).expect("valid format description");

        let formatted_dt = dt.format(&format).expect("valid date-time format");
        self.modified_ts = modified_ts;
        self.modified_str = formatted_dt;
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn set_file_name(&mut self, file_name: &str) {
        self.file_name = file_name.to_string();
    }

    pub fn set_root_dir(&mut self, root_dir: &str) {
        self.root_dir = root_dir.to_string();
    }

    pub fn get_cache_key_for_title(&self) -> String {
        format!("rusty-notes-title-{}", self.file_path)
    }
}
