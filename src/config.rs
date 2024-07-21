use figment::{Figment, Error, providers::{Env, Serialized}};
use std::{env, process};
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server_addr: String,
    pub dir_notes: String,
    pub dir_templates: String,
}

impl Default for Config {
    fn default() -> Self {
        let dir_home = if let Ok(env_var) = env::var("HOME") {
            env_var
        } else {
            println!("get env HOME failed.");
            process::exit(1);
        };

        let dir_home = Path::new(&dir_home);
        let dir_notes = dir_home.join("rusty-notes");
        let dir_templates = dir_notes.join("static/templates");
        let dir_notes = dir_notes.to_string_lossy().into_owned();
        let dir_templates = dir_templates.to_string_lossy().into_owned();

        Config {
            server_addr: "127.0.0.1:7777".into(),
            dir_notes: dir_notes.into(),
            dir_templates: dir_templates.into(),
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        let config: Config = Figment::from(Serialized::defaults(Config::default()))
            .merge(Env::raw().only(&[
               "RUSTY_SERVER_ADDR",
               "RUSTY_NOTES_DIR",
               "RUSTY_TEMPLATES_DIR",
            ]))
            .extract()?;

        Ok(config)
    }
}
