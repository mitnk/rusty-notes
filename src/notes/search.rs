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

    let mut cmd_pipe: Vec<String> = Vec::new();
    let tokens: Vec<&str> = q.split(' ').collect();

    for (i, word) in tokens.iter().enumerate() {
        let cmd = if i == 0 {
            format!("rg -g '!subusers/' -j1 -t md -il '{}' '{}'", word, notes_dir)
        } else {
            format!("xargs rg -j1 -il '{}'", word)
        };
        cmd_pipe.push(cmd);
    }

    // Execute the pipeline commands
    let mut previous_output: Option<std::process::Child> = None;

    for cmd in cmd_pipe {
        let mut command = Command::new("sh");
        command.arg("-c").arg(&cmd);

        // If there's previous output, set it as stdin for the new command using a pipe
        if let Some(mut prev_output) = previous_output {
            // Create a pipe between the commands
            command.stdin(prev_output.stdout.take().unwrap());
        }

        let child = command.stdout(Stdio::piped()).spawn().expect("Failed to execute command");

        previous_output = Some(child);
    }

    if let Some(mut final_output) = previous_output {
        let stdout = final_output.stdout.take().expect("Failed to retrieve stdout");
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            match line {
                Ok(line) if !line.trim().is_empty() && !note_list.contains(&line) => {
                    note_list.push(line.trim().to_owned());
                },
                _ => {}
            }
        }

        final_output.wait().expect("Command wasn't running");
    }
}
