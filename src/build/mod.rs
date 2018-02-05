//! This file is used for collecting the source files, preferences, and more before
//! the actual "building", or "compiling", of the binary file.

mod compilers;

use std::fs::DirBuilder;
use std::path::Path;
use project::Project;
use user::Config;
use utils;

pub fn build(release: bool, verbose: bool) -> Result<(), &'static str> {
    let project = Project::get(Path::new("."))?;

    // If this project has a build.py file but does not specifically have
    // Python as its target configuration, we just execute the file and continue
    // building.
    if Path::new("./build.py").exists() {
        if verbose {
            eprintln!("Executing build.py...");
        }
        utils::shell_command(String::from("python ./build.py"), true).expect("execute build.py");
    }

    let mut dir_builder = DirBuilder::new();
    // Recursive enables us to not get an error if the directory exists
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

    // The path to every source file in source/
    let mut sources = Vec::<String>::new();
    let mut main_extension = String::new();

    // This is where we get our source files
    for path in utils::get_files_in_directory(source_dir) {
        let ext = path.extension().unwrap();
        if path.file_stem().unwrap().to_str() == Some("main") {
            main_extension = ext.to_str().unwrap().to_owned(); // Obtain the extension of our main source file
            sources.push(path.to_str().unwrap().to_owned()); // Push the main source file
        } else {
            // When the file is not main
            if ext == "c" || ext == "cc" || ext == "cxx" || ext == "cpp" {
                // Push the source file (as long as it is a recognized source file)
                sources.push(path.to_str().unwrap().to_owned());
            }
        }
    }

    // Determine the main language used in the project
    let language: Language;
    match main_extension.as_str() {
        "c" => language = Language::C,

        "cc" => language = Language::Cpp,
        "cxx" => language = Language::Cpp,
        "cpp" => language = Language::Cpp,

        _ => return Err("Filetype of main in ./source/ does not match C or C++."),
    }

    let mut compiler_options = CompilerOptions {
        release: release,
        verbose: verbose,
        sources: sources,
        language: language,
        compiler: Compiler::GNU,
    };

    // Set the compiler
    compiler_options.compiler = match project.build.clone() {
        Some(build) => {
            // If the project configuration has a preferred compiler,
            // then forcefully use it. Otherwise, use the preferred
            // compiler specified in the user's config file.
            match build.preferred_compiler {
                Some(compiler) => compiler,
                None => Config::get()?.preferred_compiler,
            }
        }
        None => Config::get()?.preferred_compiler,
    };

    compilers::compile(project, compiler_options)?;

    Ok(())
}

/// A high-level interface for compiler options.
pub struct CompilerOptions {
    pub release: bool,
    pub verbose: bool,
    pub sources: Vec<String>,
    pub language: Language,
    pub compiler: Compiler,
}

pub enum Language {
    C,
    Cpp,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Compiler {
    GNU,
    Clang,
}
