use std::path::Path;

pub fn is_dir(path: &str) -> bool {
    let dir_path = Path::new(path);
    dir_path.exists() && dir_path.is_dir()
}
