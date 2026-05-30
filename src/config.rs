use figment::{
    providers::{Env, Serialized},
    Error, Figment,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{env, process};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub rusty_server_addr: String,
    pub rusty_notes_dir: String,
    pub rusty_url_prefix: String,
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
        let dir_notes = dir_notes.to_string_lossy().into_owned();

        Config {
            rusty_server_addr: "127.0.0.1:7777".into(),
            rusty_notes_dir: dir_notes,
            rusty_url_prefix: "/".into(),
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self, Box<Error>> {
        let config: Config = Figment::from(Serialized::defaults(Config::default()))
            .merge(Env::raw().only(&["RUSTY_SERVER_ADDR", "RUSTY_NOTES_DIR", "RUSTY_URL_PREFIX"]))
            .extract()?;

        Ok(config)
    }
}
