use project::Project;
use super::{CompilerOptions, CompilerTrait, Language};
use utils;

pub struct Clang;

impl CompilerTrait for Clang {
    fn display() -> String {
        String::from("Clang")
    }

    fn exists() -> bool {
        utils::shell_command_exists("clang -v")
    }

    fn generate_command(project: Project, compiler_options: CompilerOptions) -> String {
        let mut command = String::new();

        // Compiler name
        match compiler_options.language {
            Language::C => command.push_str("clang"),
            Language::Cpp => command.push_str("clang++"),
        }

        // Sources
        for source in compiler_options.sources {
            command.push_str(format!(" {}", source).as_str());
        }

        if cfg!(target_os = "windows") {
            if compiler_options.release {
                command.push_str(
                    format!(" -o ./target/release/{}.exe", project.package.name).as_str(),
                );
            } else {
                command
                    .push_str(format!(" -o ./target/debug/{}.exe", project.package.name).as_str());
            }
        } else {
            if compiler_options.release {
                command.push_str(format!(" -o ./target/release/{}", project.package.name).as_str());
            } else {
                command.push_str(format!(" -o ./target/debug/{}", project.package.name).as_str());
            }
        }

        // Warnings
        command.push_str(" -w");

        // Optimizations and warnings
        if compiler_options.release {
            command.push_str(" -O3");
        }

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

        if let Some(build) = project.build {
            // We just append every option that they specify in `clang_options` of [build].
            match build.clang_options {
                Some(options) => for option in options {
                    command.push_str(format!(" {}", option).as_str());
                },
                None => {}
            }
        }

        command
    }
}
