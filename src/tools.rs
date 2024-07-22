use std::path::Path;
use std::process::Command;

pub fn is_dir(path: &str) -> bool {
    let dir_path = Path::new(path);
    dir_path.exists() && dir_path.is_dir()
}

pub fn command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
