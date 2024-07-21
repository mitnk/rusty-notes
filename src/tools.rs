use std::fs;
use std::path::Path;

pub fn get_cookie_key() -> Result<String, std::io::Error> {
    let s = fs::read_to_string("/etc/rsbugs/cookie-key")?;
    Ok(s + "71c465e1-bfd5-42c4-945c-409870b347bd")
}

pub fn is_dir(path: &str) -> bool {
    let dir_path = Path::new(path);
    dir_path.exists() && dir_path.is_dir()
}
