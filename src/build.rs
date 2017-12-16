use std::fs::{self, File, DirBuilder};
use std::path::{Path, PathBuf};
use std::process::{ExitStatus, Command};
use project::{Project, Target};

pub fn build(release: bool) -> Result<(), &'static str> {

    let project = Project::get(".")?;

    // Python, like the other (future) supported scripting languages,
    // is used to custom build. This enables anyone to make any
    // kind of project they need.
    if project.get_target().expect("target configuration") == Target::Python {
        if Path::new("./build.py").exists() {
            // Execute the python file
            let mut child = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .arg("/C")
                    .arg("python .\\build.py")
                    .spawn()
                    .expect("execute build.py")
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg("python ./build.py")
                    .spawn()
                    .expect("execute build.py")
            };
            // Wait for the Python program to finish
            child.wait().unwrap();

            println!("Finished");
        } else {
            return Err("The target configuration is set to Python, but I can't find the file 'build.py' at the root of the project.");
        }
        return Ok(());
    }

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

    let mut language = String::new();

    // The path to every source file in source/
    let mut sources = Vec::<String>::new();

    for path in get_files_in_directory(Path::new("./source")) {
        let ext = path.extension().unwrap();
        if path.file_stem().unwrap().to_str() == Some("main") {
            language = ext.to_str().unwrap().to_owned();
            // NOTE: we don't push the main.(c/cpp) file, because its
            // automatically loaded by compile_c() / compile_cpp().
        } else {
            // When the file is not main
            if ext == "c" || ext == "cpp" {
                // Push the source file (as long as it is a recognized source file)
                sources.push(path.to_str().unwrap().to_owned());
            }
        }
    }

    // We only support ".c", ".cpp" extensions (besides custom)
    match language.as_str() {
        "c" => compile(project, release, sources, Language::C),
        "cpp" => compile(project, release, sources, Language::Cpp),
        _ => return Err("Unknown sources. If you're using a custom build, please use Python."),
    }
    Ok(())
}

fn get_files_in_directory(directory: &Path) -> Vec<PathBuf> {
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

/// Executes a shell command in the background
pub fn shell_command(command: String) -> Result<(), ::std::io::Error> {
    // Turn the String into a Vec<String>
    if cfg!(target_os = "windows") {
        let vector: Vec<String> = windows_path(command).split_whitespace().map(|s| s.to_owned()).collect();

        let status = Command::new("cmd")
            .arg("/C")
            .args(vector.as_slice())
            .spawn()?;
        
        Ok(())
    } else {
        let vector: Vec<String> = command.split_whitespace().map(|s| s.to_owned()).collect();

        let status = Command::new("sh")
            .arg("-c")
            .args(vector.as_slice())
            .spawn()?;

        Ok(())
    }
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

pub enum Language {
    C,
    Cpp,
}

fn compile(project: Project, release: bool, sources: Vec<String>, language: Language) {
    let mut command = String::new();
    // TODO: push compiler name depending on choice
    match language {
        Language::C => {
            // Compiler
            command.push_str("gcc");
            command.push_str(" ./source/main.c");
        },
        Language::Cpp => {
            // Compiler
            command.push_str("g++");
            command.push_str(" ./source/main.cpp");
        },
    }

    for source in sources {
        command.push_str(format!(" {}", source).as_str());
    }

    if cfg!(target_os = "windows") {
        command.push_str(format!(" -o ./target/debug/{}.exe", project.package.name).as_str());
    } else {
        command.push_str(format!(" -o ./target/debug/{}", project.package.name).as_str());
    }

    match shell_command(command) {
        Err(e) => println!("{}", e),
        _ => {}
    }
}
