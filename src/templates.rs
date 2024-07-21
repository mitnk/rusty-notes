use lazy_static::lazy_static;
use std::path::Path;
use tera::Tera;

use crate::config::Config;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let config = Config::from_env().unwrap();
        let path = Path::new(&config.rusty_dir_templates);
        let dir_templates = path.join("**/*");
        let dir_templates = dir_templates.to_string_lossy();
        let tera = match Tera::new(&dir_templates) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}
