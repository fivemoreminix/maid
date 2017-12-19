use std::fs::DirBuilder;
use std::path::Path;
use project::{Project, Target};
use utils;

pub fn build(release: bool) -> Result<(), &'static str> {

    let project = Project::get(".")?;

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
        utils::shell_command(String::from("python ./build.py")).expect("execute build.py");
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

    let language: Language;
    match main_extension.as_str() {
        "c" => language = Language::C,
        
        "cc" => language = Language::Cpp,
        "cxx" => language = Language::Cpp,
        "cpp" => language = Language::Cpp,

        _ => return Err("Filetype of main in ./source/ does not match C or C++."),
    }

    compile_gcc(project, CompilerOptions {
        release: release,
        sources: sources,
        language: language,
    });
    
    Ok(())
}
pub enum Language {
    C,
    Cpp,
}

/// A high-level interface for compiler options.
pub struct CompilerOptions {
    pub release: bool,
    pub sources: Vec<String>,
    pub language: Language,
}

fn compile_gcc(project: Project, compiler_options: CompilerOptions) {
    let mut command = String::new();
    match compiler_options.language {
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

    for source in compiler_options.sources {
        command.push_str(format!(" {}", source).as_str());
    }

    if cfg!(target_os = "windows") {
        if compiler_options.release {
            command.push_str(format!(" -o ./target/release/{}.exe", project.package.name).as_str());
        } else {
            command.push_str(format!(" -o ./target/debug/{}.exe", project.package.name).as_str());
        }
    } else {
        if compiler_options.release {
            command.push_str(format!(" -o ./target/release/{}", project.package.name).as_str());
        } else {
            command.push_str(format!(" -o ./target/debug/{}", project.package.name).as_str());
        }
    }

    if compiler_options.release {
        command.push_str(" -O3");
    }

    // All warnings
    command.push_str(" -w");

    // Preprocessor defines
    command.push_str(format!(" -D PACKAGE_NAME=\"{}\" -D PACKAGE_VERSION=\"{}\"", project.package.name, project.package.version).as_str());

    match utils::shell_command(command) {
        Err(e) => println!("{}", e),
        _ => {}
    }
}

fn compile_clang(project: Project, compiler_options: CompilerOptions) {
    unimplemented!();
}
