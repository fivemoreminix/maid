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

    match language.as_str() {
        "c" => compile_c(),
        "cpp" => return,
        "cxx" => return,
        "cc" => return,
        _ => return,
    }
}

/// Executes a shell command in the background
fn shell_command(command: String) -> Result<ExitStatus, ::std::io::Error> {
    // Turn the String into a Vec<String>
    let vector: Vec<String> = command.split_whitespace().map(|s| s.to_owned()).collect();

    if cfg!(target_os = "windows") {
        let status = Command::new("cmd")
            .arg("/C")
            .args(vector.as_slice())
            .status()?;
        Ok(status)
    } else {
        let status = Command::new("sh")
            .arg("-c")
            .args(vector.as_slice())
            .status()?;
        Ok(status)
    }
}

fn compile_c() {
    let command = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg("gcc")
            .arg(".\\source\\main.c")
            .arg("-o")
            .arg(".\\target\\debug\\main.exe")
            .output()
            .expect("failed to execute gcc")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("gcc")
            .arg("./source/main.c")
            .arg("-o")
            .arg("./target/debug/main")
            .output()
            .expect("failed to execute gcc")
    };

    let stdout = String::from_utf8(command.stdout).unwrap();
    let stderr = String::from_utf8(command.stderr).unwrap();
    println!("Stdout: {:?}\nStderr: {:?}", stdout, stderr);
}
