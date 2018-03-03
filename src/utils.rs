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

/// Executes a shell command in the background
pub fn shell_command(
    command: &str,
    catch_exit_codes: bool,
) -> Result<ExitStatus, ::std::io::Error> {
    let mut status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .args(string_to_vec(&windows_path(command)).as_slice())
            .spawn()?
    } else {
        Command::new("sh")
            .arg("-c")
            .args(string_to_vec(command).as_slice())
            .spawn()?
    };

    let result = status.wait().unwrap();

    if catch_exit_codes && result.code().unwrap() != 0 {
        let code = result.code().unwrap();
        return Err(::std::io::Error::new(
            ::std::io::ErrorKind::Other,
            format!("process exited with code: {}", code),
        ));
    }

    Ok(result)
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
