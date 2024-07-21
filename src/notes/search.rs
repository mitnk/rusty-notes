use log::info;


pub fn search_notes_by_fd(q: &str, notes_dir: &str, note_list: &mut Vec<String>) {
    let cmd = format!("/usr/local/bin/fd -E 'subusers/' '{}' '{}'", q, notes_dir);
    let result = cicada::run(&cmd);
    if result.stdout.is_empty() {
        return;
    } else {
        for line in result.stdout.split('\n') {
            if line.trim().is_empty() || !line.ends_with(".md") {
                continue;
            }
            if note_list.iter().find(|x| *x == line).is_some() {
                continue;
            }
            note_list.push(line.trim().to_owned());
        }
    }
}

pub fn search_notes_by_rg(q: &str, notes_dir: &str, note_list: &mut Vec<String>) {
    let mut cmd_pipe: Vec<String> = Vec::new();
    let tokens: Vec<&str> = q.split(' ').collect();
    for (i, word) in tokens.iter().enumerate() {
        let cmd = if i == 0 {
            format!("/usr/local/bin/rg -g '!subusers/' -j1 -t md -il '{}' '{}'", word, notes_dir)
        } else {
            format!("xargs /usr/local/bin/rg -j1 -il '{}'", word)
        };
        cmd_pipe.push(cmd)
    }

    let cmd_line = cmd_pipe.join(" | ");
    info!("searching with cmd: {:?}", cmd_line);
    let result = cicada::run(&cmd_line);
    if result.stdout.is_empty() {
        return;
    } else {
        for line in result.stdout.split('\n') {
            if line.trim().is_empty() {
                continue;
            }
            if note_list.iter().find(|x| *x == line).is_some() {
                continue;
            }
            note_list.push(line.trim().to_owned());
        }
    }
}
