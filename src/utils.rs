use std::process::{Command, ExitStatus};
use std::path::{Path, PathBuf};
use std::fs;

pub fn string_to_vec(string: &str) -> Vec<String> {
    string.split_whitespace().map(|s| s.to_owned()).collect()
}

/// Takes a String and turns all occurrences of '/' into '\'.
pub fn windows_path(path: &str) -> String {
    path.chars()
        .map(|c| if c == '/' { '\\' } else { c })
        .collect()
}

/// Executes a shell command and waits for it to finish
pub fn shell_command(command: &str, silent: bool) -> Result<ExitStatus, ::std::io::Error> {
    let command = if cfg!(windows) {
        string_to_vec(&windows_path(command))
    } else {
        string_to_vec(&command)
    };

    let result = if silent {
        Command::new(&command[0])
            .args(&command[1..])
            .output()?
            .status
    } else {
        if cfg!(target_os = "windows") {
            Command::new("cmd").arg("/C").args(command).spawn()?.wait()?
        } else {
            Command::new("sh").arg("-c").args(command).spawn()?.wait()?
        }
    };

    Ok(result)
}

pub fn shell_command_exists(command: &str) -> bool {
    shell_command(command, true).unwrap().success()
}

pub fn get_files_in_directory(directory: &Path, recursive: bool) -> Vec<PathBuf> {
    assert!(directory.is_dir());

    let mut files = Vec::<PathBuf>::new();

    // Read every entry in the directory, exporting them to a Vec.
    fs::read_dir(directory).unwrap().for_each(|entry| {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files.push(path.clone());
            /* When we recursively scan, we also include all entries
            from subdirectories. */
            if recursive {
                for p in get_files_in_directory(&path, true) {
                    files.push(p);
                }
            }
        } else {
            files.push(path);
        }
    });

    files
}
