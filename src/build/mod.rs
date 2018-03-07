//! This file is used for collecting the source files, preferences, and more before
//! the actual "building", or "compiling", of the binary file.

mod gcc;
mod clang;

use std::fs::DirBuilder;
use std::path::Path;
use project::Project;
use user::Config;
use utils;
use ansi_term::Color::Green;

pub fn build(release: bool, verbose: bool) -> Result<(), CompileError> {
    let project = match Project::get() {
        Ok(project) => project,
        Err(e) => {
            return Err(CompileError {
                error_type: CompileErrorType::CouldNotLocateProjectFile,
                msg: e.description,
            })
        }
    };

    // If this project has a build.py file but does not specifically have
    // Python as its target configuration, we just execute the file and continue
    // building.
    if Path::new("./build.py").exists() {
        if verbose {
            eprintln!("Executing build.py...");
        }

        if utils::shell_command("python ./build.py", false)
            .expect("Failed to execute Python.")
            .success() == false
        {
            return Err(CompileError {
                error_type: CompileErrorType::PythonBuildScriptReturnedNonZero,
                msg: "Python build script returned non zero.".to_string(),
            });
        }
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
    for path in utils::get_files_in_directory(source_dir, true) {
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

        _ => {
            return Err(CompileError {
                error_type: CompileErrorType::FileTypeOfMainNotRecognized,
                msg: "File extension of 'main' in './source/' does not match C or C++.".to_string(),
            })
        }
    }

    let compiler_options = CompilerOptions {
        release: release,
        verbose: verbose,
        sources: sources,
        language: language,
    };

    // Set the compiler
    let compiler: Compiler = match project.build.clone() {
        Some(build) => {
            // If the project configuration has a preferred compiler,
            // then forcefully use it. Otherwise, use the preferred
            // compiler specified in the user's config file.
            match build.preferred_compiler {
                Some(compiler) => compiler,
                None => match Config::get() {
                    Ok(config) => config.preferred_compiler,
                    Err(e) => {
                        return Err(CompileError {
                            error_type: CompileErrorType::CouldNotReadUserConfig,
                            msg: e.to_string(),
                        })
                    }
                },
            }
        }
        None => match Config::get() {
            Ok(config) => config.preferred_compiler,
            Err(e) => {
                return Err(CompileError {
                    error_type: CompileErrorType::CouldNotReadUserConfig,
                    msg: e.to_string(),
                })
            }
        },
    };

    match compiler {
        Compiler::GNU => compile(gcc::GCC, project, compiler_options),
        Compiler::Clang => compile(clang::Clang, project, compiler_options),
    }
}

/// A high-level interface for compiler options.
#[derive(Clone)]
pub struct CompilerOptions {
    pub release: bool,
    pub verbose: bool,
    pub sources: Vec<String>,
    pub language: Language,
    // pub compiler: Compiler,
}

#[derive(Clone)]
pub enum Language {
    C,
    Cpp,
}

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone, Debug)]
pub enum Compiler {
    GNU,
    Clang,
    // MSVC,
}

pub fn detect_available_compilers() -> Vec<Compiler> {
    let mut compilers = Vec::<Compiler>::new();

    if gcc::GCC::exists() {
        compilers.push(Compiler::GNU)
    }
    if clang::Clang::exists() {
        compilers.push(Compiler::Clang)
    }

    compilers
}

#[derive(Debug)]
pub enum CompileErrorType {
    CompilerReturnedNonZero,
    CouldNotLocateProjectFile,
    PythonBuildScriptReturnedNonZero,
    FileTypeOfMainNotRecognized,
    CouldNotReadUserConfig,
}

#[derive(Debug)]
pub struct CompileError {
    pub error_type: CompileErrorType,
    pub msg: String,
}

pub trait CompilerTrait {
    /// Must return the name of the compiler spelled properly using capitals and
    /// punctuation, if applicable. Examples: GNU, Clang, MSVC.
    #[inline]
    fn display() -> String;

    /// Must execute a command that ensures the compiler is in PATH, is executable,
    /// and returns zero on execution. This command MUST return zero/true for the compiler
    /// to be considered "found".
    #[inline]
    fn exists() -> bool;

    /// Must generate an entire build command given the information about the project
    /// and build settings.
    fn generate_command(project: Project, compiler_options: CompilerOptions) -> String;
}

pub fn compile<T>(
    _: T,
    project: Project,
    compiler_options: CompilerOptions,
) -> Result<(), CompileError>
where
    T: CompilerTrait,
{
    let command: String = T::generate_command(project.clone(), compiler_options.clone());

    if compiler_options.verbose {
        eprintln!("{}", command);
    }

    println!(
        "   {} {} v{} with {}",
        Green.paint("Compiling"),
        project.package.name,
        project.package.version,
        T::display(),
    );

    // Calling the compiler with our command
    if utils::shell_command(&command, false)
        .expect("Failed to query compiler.")
        .success() == false
    {
        return Err(CompileError {
            error_type: CompileErrorType::CompilerReturnedNonZero,
            msg: "Compilation terminated due to previous error(s).".to_string(),
        });
    }

    if compiler_options.release {
        println!("    {} release [optimized]", Green.paint("Finished"));
    } else {
        println!("    {} debug [unoptimized]", Green.paint("Finished"));
    }

    Ok(())
}
