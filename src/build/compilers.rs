//! The code to implement compilers is very messy so we keep all of our compiler-specific code here.

use super::{CompilerOptions, Language};
use project::{Project, Target};
use utils;
use super::Compiler;
use ansi_term::Color::Green;

pub fn compile(
    project: Project,
    compiler_options: CompilerOptions,
) -> Result<(), &'static str> {
    let mut command = String::new();

    // Set the compiler to use and add the main.(c/cpp) source file
    match compiler_options.language {
        Language::C => {
            match compiler_options.compiler {
                Compiler::GNU => {
                    command.push_str("gcc");
                },
                Compiler::Clang => {
                    command.push_str("clang");
                }
            }
            command.push_str(" ./source/main.c");
        },
        Language::Cpp => {
            match compiler_options.compiler {
                Compiler::GNU => {
                    command.push_str("g++");
                },
                Compiler::Clang => {
                    command.push_str("clang++");
                }
            }
            command.push_str(" ./source/main.cpp");
        }
    }

    for source in compiler_options.sources {
        command.push_str(format!(" {}", source).as_str());
    }

    if project.package.target == Target::Static {
        match compiler_options.compiler {
            Compiler::GNU => command.push_str(" -c"),
            _ => unimplemented!(),
        }
    }

    match project.package.target {
        Target::Static => if compiler_options.release {
            command.push_str(format!(" -o ./target/release/{}.o", project.package.name).as_str());
        } else {
            command.push_str(format!(" -o ./target/debug/{}.o", project.package.name).as_str());
        }
        Target::Dynamic => unimplemented!(),
        _ => {
            if cfg!(target_os = "windows") {
                if compiler_options.release {
                    command.push_str(
                        format!(" -o ./target/release/{}.exe", project.package.name).as_str()
                    );
                } else {
                    command.push_str(
                        format!(" -o ./target/debug/{}.exe", project.package.name).as_str()
                    );
                }
            } else {
                if compiler_options.release {
                    command.push_str(format!(" -o ./target/release/{}", project.package.name).as_str());
                } else {
                    command.push_str(format!(" -o ./target/debug/{}", project.package.name).as_str());
                }
            }
        }
    }

    // Optimizations and warnings
    if compiler_options.release {
        command.push_str(" -O3");
    } else {
        command.push_str(" -w");
    }

    // Preprocessor defines
    match compiler_options.compiler {
        Compiler::GNU => {
            command.push_str(
                format!(
                    " -D MAID_PACKAGE_NAME=\"{}\" -D MAID_PACKAGE_VERSION=\"{}\"",
                    project.package.name, project.package.version
                ).as_str(),
            );
            if compiler_options.release {
                command.push_str(" -D MAID_RELEASE");
            } else {
                command.push_str(" -D MAID_DEBUG");
            }
        }
        Compiler::Clang => {
            command.push_str(
                format!(
                    " -DMAID_PACKAGE_NAME=\"{}\" -DMAID_PACKAGE_VERSION=\"{}\"",
                    project.package.name, project.package.version
                ).as_str(),
            );
            if compiler_options.release {
                command.push_str(" -DMAID_RELEASE");
            } else {
                command.push_str(" -DMAID_DEBUG");
            }
        }
    }

    if let Some(dependencies) = project.dependencies {
        // Header search directories
        match dependencies.header_search_directories {
            Some(directories) => for directory in directories {
                command.push_str(format!(" -I {}", directory).as_str());
            },
            None => {}
        }

        // Linker search directories
        match dependencies.linker_search_directories {
            Some(directories) => for directory in directories {
                command.push_str(format!(" -L {}", directory).as_str());
            },
            None => {}
        }

        // The "linker search directories" are just used to point to a directory where the following
        // "link library" name is passed. For example, in the directory `./SDL2/lib`, there may be a file
        // called "libSDL2.lib", and you have one "link library", called "SDL2", so " -lSDL2". The linker
        // finds the file "libSDL2.lib", in the "linker search directory" (" -L ./SDL2/lib").
        match dependencies.link_libraries {
            Some(libraries) => for name in libraries {
                command.push_str(format!(" -l{}", name).as_str());
            },
            None => {}
        }
    }

    // Custom compiler options that can be specified within the project file
    if let Some(build) = project.build {
        match compiler_options.compiler {
            Compiler::GNU => {
                // We just append every option that they specify in `gnu_options` of [build].
                match build.gnu_options {
                    Some(options) => for option in options {
                        command.push_str(format!(" {}", option).as_str());
                    },
                    None => {}
                }
            }
            Compiler::Clang => {
                // We just append every option that they specify in `gnu_options` of [build].
                match build.clang_options {
                    Some(options) => for option in options {
                        command.push_str(format!(" {}", option).as_str());
                    },
                    None => {}
                }
            }
        }
    }

    // Verbose
    if compiler_options.verbose {
        eprintln!("{}", command);
    }

    // Calling the compiler with our command
    println!(
        "\t{} {} v{} with GNU",
        Green.paint("Compiling"), project.package.name, project.package.version
    );
    match utils::shell_command(command, true) {
        Err(_) => return Err("Compilation terminated due to previous error(s)."),
        _ => {}
    }

    // If we're working with a static library, we need to make an archive of the .o files
    if project.package.target == Target::Static {
        if compiler_options.release {
            match compiler_options.compiler {
                Compiler::GNU => {
                    if let Err(_) = utils::shell_command(
                        format!(
                            "ar rcs ./target/release/lib{}.a ./target/release/{}.o",
                            project.package.name, project.package.name
                        ),
                        true,
                    ) {
                        return Err("compilation terminated due to previous error(s)");
                    }
                }
                Compiler::Clang => {
                    unimplemented!();
                }
            }
        } else {
            match compiler_options.compiler {
                Compiler::GNU => {
                    if let Err(_) = utils::shell_command(
                        format!(
                            "ar rcs ./target/debug/lib{}.a ./target/debug/{}.o",
                            project.package.name, project.package.name
                        ),
                        true,
                    ) {
                        return Err("Compilation terminated due to previous error(s).");
                    }
                }
                Compiler::Clang => {
                    unimplemented!();
                }
            }
        }
    }

    if compiler_options.release {
        println!(
            "\t {} release [optimized]",
            Green.paint("Finished"),
        );
    } else {
        println!(
            "\t {} debug [unoptimized]",
            Green.paint("Finished"),
        );
    }

    Ok(())
}