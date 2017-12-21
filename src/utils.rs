use std::process::{ExitStatus, Command};
use std::path::{Path, PathBuf};
use std::fs;

pub fn string_to_vec(string: String) -> Vec<String> {
    string.split_whitespace().map(|s| s.to_owned()).collect()
}

/// Takes a String and turns all occurrences of '/' into '\'.
pub fn windows_path(path: String) -> String {
    // Create an iterator over the characters in the path
    let chars = path.chars();
    // Make an empty String
    let mut new_path = String::new();
    // Iterate over every character in the path
    for c in chars {
        if c == '/' {
            // If a character is a forward slash, push the Windows version
            new_path.push('\\');
        } else {
            // If it's not a forward slash, just push the character
            new_path.push(c);
        }
    }
    new_path
}

pub fn print_error_str(error: &str) {
    eprintln!("\nerror: {}", error);
}

pub fn print_error(error: String) {
    eprintln!("\nerror: {}", error);
}

/// Executes a shell command in the background
pub fn shell_command(command: String, catch_exit_codes: bool) -> Result<ExitStatus, ::std::io::Error> {
    let mut status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .args(string_to_vec(windows_path(command)).as_slice())
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
        print_error(format!("process exited with code: {}", code));
        return Err(::std::io::Error::new(::std::io::ErrorKind::Other, format!("process exited with code: {}", code)));
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
