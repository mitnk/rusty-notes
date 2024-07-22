use log::info;
use crate::tools;
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};


pub fn search_notes_by_fd(q: &str, notes_dir: &str, note_list: &mut Vec<String>) {
    if !tools::command_exists("fd") {
        info!("WARN: fd not found");
        return;
    }

    let output = Command::new("fd")
        .arg("-E")
        .arg("subusers/")
        .arg(q)
        .arg(notes_dir)
        .output()
        .expect("Failed to execute command");

    if output.stdout.is_empty() {
        return;
    } else {
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        for line in stdout_str.lines() {
            if line.trim().is_empty() || !line.ends_with(".md") {
                continue;
            }
            if note_list.iter().any(|x| *x == line) {
                continue;
            }
            note_list.push(line.trim().to_owned());
        }
    }
}

pub fn search_notes_by_rg(q: &str, notes_dir: &str, note_list: &mut Vec<String>) {
    if !tools::command_exists("rg") {
        info!("WARN: `rg` not found");
        return;
    }
    if !tools::command_exists("bash") {
        info!("WARN: `bash` not found; rg search stopped");
        return;
    }

    let mut cmd_pipe: Vec<String> = Vec::new();
    let tokens: Vec<&str> = q.split(' ').collect();

    for (i, word) in tokens.iter().enumerate() {
        let cmd = if i == 0 {
            format!("rg --follow -j1 -t md -il '{}' '{}'", word, notes_dir)
        } else {
            format!("xargs rg -j1 -il '{}'", word)
        };
        cmd_pipe.push(cmd);
    }

    if cmd_pipe.is_empty() {
        return;
    }

    let mut current_output = Command::new("bash")
        .arg("-c")
        .arg(&cmd_pipe[0])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute first rg command")
        .stdout
        .expect("Failed to capture stdout");

    for cmd in cmd_pipe.iter().skip(1) {
        let process = Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .stdin(Stdio::from(current_output))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute command in pipeline");

        current_output = process.stdout.expect("Failed to capture stdout");
    }

    let reader = BufReader::new(current_output);
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.trim().is_empty() || !line.ends_with(".md") {
                continue;
            }
            if note_list.iter().any(|x| *x == line) {
                continue;
            }
            note_list.push(line.trim().to_owned());
        }
    }
}
