use std::fs::DirBuilder;
use std::path::Path;
use project::{Project, Target};
use utils;

pub fn build(release: Release) -> Result<(), &'static str> {

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
    if release == Release::Release {
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

    // This is where we get our source files
    for path in utils::get_files_in_directory(Path::new("./source")) {
        let ext = path.extension().unwrap();
        if path.file_stem().unwrap().to_str() == Some("main") {
            language = ext.to_str().unwrap().to_owned();
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

    // We only support ".c", ".cpp" extensions (besides custom)
    match language.as_str() {
        "c" => compile(project, release, sources, Language::C),
        "cpp" => compile(project, release, sources, Language::Cpp),
        _ => return Err("Unknown sources. If you're using a custom build, please use Python."),
    }
    Ok(())
}
pub enum Language {
    C,
    Cpp,
}

#[derive(PartialEq)]
/// For easily making new Release options.
pub enum Release {
    Release,
    Debug,
}

fn compile(project: Project, release: Release, sources: Vec<String>, language: Language) {
    let mut command = String::new();
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
        if release == Release::Debug {
            command.push_str(format!(" -o ./target/debug/{}.exe", project.package.name).as_str());
        } else {
            command.push_str(format!(" -o ./target/release/{}.exe", project.package.name).as_str());
        }
    } else {
        if release == Release::Debug {
            command.push_str(format!(" -o ./target/debug/{}", project.package.name).as_str());
        } else {
            command.push_str(format!(" -o ./target/release/{}", project.package.name).as_str());
        }
    }

    match release {
        Release::Release => command.push_str(" -O3"),
        _ => {},
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
