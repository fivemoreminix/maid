//! This file is used for collecting the source files, preferences, and more before
//! the actual "building", or "compiling", of the binary file.

mod compilers;

use std::fs::DirBuilder;
use std::path::Path;
use project::{Project, Target};
use user::{Compiler, Config};
use utils;

pub fn build(release: bool, verbose: bool) -> Result<(), &'static str> {

    let project = Project::get(Path::new("."))?;

    // Python, like the other (future) supported scripting languages,
    // is used to custom build. This enables anyone to make any
    // kind of project they need.
    if project.package.target == Target::Python {
        if Path::new("./build.py").exists() {
            // Execute the python file
            utils::shell_command(String::from("python ./build.py")).expect("execute build.py");

            println!("Finished");
        } else {
            return Err("The target configuration is set to Python, but I can't find the file 'build.py' at the root of the project.");
        }
        return Ok(());
    }

    // If this project has a build.py file but does not specifically have
    // Python as its target configuration, we just execute the file and continue
    // building.
    if Path::new("./build.py").exists() {
        if verbose {
            eprintln!("Executing build.py...");
        }
        utils::shell_command(String::from("python ./build.py")).expect("execute build.py");
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
            main_extension = ext.to_str().unwrap().to_owned();
            // NOTE: we don't push the main.(c/cpp) file, because its
            // automatically loaded by compile().
        } else {
            // When the file is not main
            if ext == "c" || ext == "cpp" {
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

    let compiler_options = CompilerOptions {
        release: release,
        verbose: verbose,
        sources: sources,
        language: language,
    };

    // If the project configuration has a preferred compiler,
    // then forcefully use it. Otherwise, use the preferred
    // compiler specified in the user's config file.
    match project.build.preferred_compiler {
        Some(compiler) => match compiler {
            Compiler::GNU => compilers::compile_gnu(project, compiler_options),
            Compiler::Clang => compilers::compile_clang(project, compiler_options),
        },
        None => match Config::get()?.preferred_compiler {
            Compiler::GNU => compilers::compile_gnu(project, compiler_options),
            Compiler::Clang => compilers::compile_clang(project, compiler_options),
        },
    }

    Ok(())
}

#[derive(Debug)]
pub enum Language {
    C,
    Cpp,
}

#[derive(Debug)]
/// A high-level interface for compiler options.
pub struct CompilerOptions {
    pub release: bool,
    pub verbose: bool,
    pub sources: Vec<String>,
    pub language: Language,
}
