//! The code to implement compilers is very messy so we keep all of our compiler-specific code here.

use super::{CompilerOptions, Language};
use project::{Target, Project};
use utils;

pub fn compile_gnu(project: Project, compiler_options: CompilerOptions) -> Result<(), &'static str> {
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

    if project.package.target == Target::Static {
        command.push_str(" -c");
    }

    if project.package.target == Target::Static {
        if compiler_options.release {
            command.push_str(format!(" -o ./target/release/{}.o", project.package.name).as_str());
        } else {
            command.push_str(format!(" -o ./target/debug/{}.o", project.package.name).as_str());
        }
    } else if project.package.target == Target::Dynamic {
        unimplemented!();
    } else {
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
    }

    if compiler_options.release {
        command.push_str(" -O3");
    }

    // All warnings
    command.push_str(" -w");

    // Preprocessor defines
    command.push_str(format!(" -D PACKAGE_NAME=\"{}\" -D PACKAGE_VERSION=\"{}\"", project.package.name, project.package.version).as_str());
    if compiler_options.release {
        command.push_str(" -D RELEASE");
    } else {
        command.push_str(" -D DEBUG");
    }

    // Header search directories
    for directory in project.dependencies.header_search_directories {
        command.push_str(format!(" -I {}", directory).as_str());
    }

    // Linker search directories
    for directory in project.dependencies.linker_search_directories {
        command.push_str(format!(" -L {}", directory).as_str());
    }

    // The "linker search directories" are just used to point to a directory where the following
    // "link library" name is passed. For example, in the directory `./SDL2/lib`, there may be a file
    // called "libSDL2.lib", and you have one "link library", called "SDL2", so " -lSDL2". The linker
    // finds the file "libSDL2.lib", in the "linker search directory" (" -L ./SDL2/lib").
    for name in project.dependencies.link_libraries {
        command.push_str(format!(" -l{}", name).as_str());
    }

    // We just append every option that they specify in `gnu_options` of [build].
    for option in project.build.gnu_options {
        command.push_str(format!(" {}", option).as_str());
    }

    if compiler_options.verbose {
        eprintln!("{}", command);
    }

    println!("\tCompiling {} v{} with GNU", project.package.name, project.package.version);
    match utils::shell_command(command, true) {
        Err(_) => return Err("compilation terminated due to previous error(s)"),
        _ => {},
    }

    // If we're working with a static library, we need to make an archive of the .o files
    if project.package.target == Target::Static {
        if compiler_options.release {
            if let Err(_) = utils::shell_command(format!("ar rcs ./target/release/lib{}.a ./target/release/{}.o",
                                                         project.package.name, project.package.name), true) {
                return Err("compilation terminated due to previous error(s)")
            }
        } else {
            if let Err(_) = utils::shell_command(format!("ar rcs ./target/debug/lib{}.a ./target/debug/{}.o",
                                                         project.package.name, project.package.name), true) {
                return Err("compilation terminated due to previous error(s)")
            }
        }
    }

    if compiler_options.release {
        println!("\t Finished release [optimized]");
    } else {
        println!("\t Finished debug [unoptimized]");
    }

    Ok(())
}

pub fn compile_clang(project: Project, compiler_options: CompilerOptions) -> Result<(), &'static str> {
    let mut command = String::new();
    match compiler_options.language {
        Language::C => {
            // Compiler
            command.push_str("clang");
            command.push_str(" ./source/main.c");
        },
        Language::Cpp => {
            // Compiler
            command.push_str("clang++");
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
    command.push_str(format!(" -DPACKAGE_NAME=\"{}\" -DPACKAGE_VERSION=\"{}\"", project.package.name, project.package.version).as_str());

    // Header search directories
    for directory in project.dependencies.header_search_directories {
        command.push_str(format!(" -I {}", directory).as_str());
    }

    // Linker search directories
    for directory in project.dependencies.linker_search_directories {
        command.push_str(format!(" -L {}", directory).as_str());
    }

    // The "linker search directories" are just used to point to a directory where the following
    // "link library" name is passed. For example, in the directory `./SDL2/lib`, there may be a file
    // called "libSDL2.lib", and you have one "link library", called "SDL2", so " -lSDL2". The linker
    // finds the file "libSDL2.lib", in the "linker search directory" (" -L ./SDL2/lib").
    for name in project.dependencies.link_libraries {
        command.push_str(format!(" -l{}", name).as_str());
    }

    // We just append every option that they specify in `gnu_options` of [build].
    for option in project.build.clang_options {
        command.push_str(format!(" {}", option).as_str());
    }

    if compiler_options.verbose {
        eprintln!("{}", command);
    }

    println!("\tCompiling {} v{} with Clang", project.package.name, project.package.version);
    match utils::shell_command(command, true) {
        Err(_) => return Err("compilation terminated due to previous error(s)"),
        _ => {}
    }

    if compiler_options.release {
        println!("\t Finished release [optimized]");
    } else {
        println!("\t Finished debug [unoptimized]");
    }

    Ok(())
}
