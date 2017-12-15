use std::fs::{self, DirBuilder};
use std::path::Path;
use std::process::{ExitStatus, Command};
use project;

pub fn build(release: bool) {
    let project = project::Project::get(".");

    let mut dir_builder = DirBuilder::new();
    dir_builder.recursive(true);
    if release {
        // Make the release folder
        dir_builder.create("./target/release").unwrap();
    } else {
        // Make the debug folder
        dir_builder.create("./target/debug").unwrap();
    }

    // Get the source folder path
    let source_dir = Path::new("./source");
    // Ensure the path is correct, and that it is a directory
    assert!(source_dir.is_dir());

    // Indecisively select custom
    let mut language = String::from("custom");

    // Find the dominant language used. (Extension of the main file)
    for entry in fs::read_dir(source_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            // TODO: consider use
            unimplemented!();
        } else {
            // Only do this for files that have extensions
            if let Some(ext) = path.extension() {
                if path.file_stem().unwrap().to_str() == Some("main") {
                    // The language we're using is the extension of the main file
                    language = ext.to_str().unwrap().to_owned();
                }
            }
        }
    }

    // We only support ".c", ".cpp", and ".asm" extensions (besides custom)
    match language.as_str() {
        "c" => compile_c(),
        "cpp" => compile_cpp(),
        _ => return,
    }
}

/// Executes a shell command in the background
fn shell_command(command: String) -> Result<ExitStatus, ::std::io::Error> {
    // Turn the String into a Vec<String>
    if cfg!(target_os = "windows") {
        let vector: Vec<String> = unix_to_windows_path(command).split_whitespace().map(|s| s.to_owned()).collect();

        let status = Command::new("cmd")
            .arg("/C")
            .args(vector.as_slice())
            .status()?;
        Ok(status)
    } else {
        let vector: Vec<String> = command.split_whitespace().map(|s| s.to_owned()).collect();

        let status = Command::new("sh")
            .arg("-c")
            .args(vector.as_slice())
            .status()?;
        Ok(status)
    }
}

fn unix_to_windows_path(path: String) -> String {
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

fn compile_cpp() {
    let command = String::from("g++ ./source/main.cpp -o ./target/debug/main.exe");
    match shell_command(command) {
        Ok(status) => {
            if (status.success()) {
                println!("Finished");
            } else {
                println!("Error compiling!");
            }
        },
        Err(e) => println!("{}", e);
    }
}

fn compile_c() {
    let command = String::from("gcc ./source/main.c -o ./target/debug/main.exe");
    match shell_command(command) {
        Ok(status) => {
            if (status.success()) {
                println!("Finished");
            } else {
                println!("Error compiling!");
            }
        },
        Err(e) => println!("{}", e),
    }
}
