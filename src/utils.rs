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
pub fn shell_command(
    command: &str,
    catch_exit_codes: bool,
    silent: bool,
) -> Result<ExitStatus, ::std::io::Error> {
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
            Command::new("cmd")
                .arg("/C")
                .args(command)
                .spawn()?
                .wait()
                .unwrap()
        } else {
            Command::new("sh")
                .arg("-c")
                .args(command)
                .spawn()?
                .wait()
                .unwrap()
        }
    };

    if catch_exit_codes && result.code().unwrap() != 0 {
        let code = result.code().unwrap();
        return Err(::std::io::Error::new(
            ::std::io::ErrorKind::Other,
            format!("Process exited with code: {}.", code),
        ));
    }

    Ok(result)
}

pub fn shell_command_exists(command: &str) -> bool {
    let result = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .args(&string_to_vec(&windows_path(command)))
            .output()
            .expect("Could not spawn a shell to locate available compilers.")
            .status
    } else {
        Command::new("sh")
            .arg("-c")
            .args(&string_to_vec(&command))
            .status()
            .expect("Could not spawn a shell to locate available compilers.")
    };

    if cfg!(windows) {
        if result.code().unwrap() == 1 {
            false
        } else {
            true
        }
    } else {
        false
    }
}

pub fn get_files_in_directory(directory: &Path) -> Vec<PathBuf> {
    // It must be a directory
    assert!(directory.is_dir());

    let mut files = Vec::<PathBuf>::new();

    // We will get a list of entires in the directory
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            // What do we do with a directory?
        } else {
            // It's a file, so we add it
            files.push(path);
        }
    }
    files
}
