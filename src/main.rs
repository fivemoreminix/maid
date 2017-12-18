extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod build;
mod project;

use structopt::StructOpt;
use project::{Project, Target};
use build::Release;
use std::process::Command;

#[derive(StructOpt, Debug)]
#[structopt(name = "maid", about = "A modern project manager for C, C++, and anything else.")]
enum Options {
    #[structopt(name = "new")]
    /// Creates a new project folder in the current directory
    New {
        #[structopt(long = "lib")]
        /// Generates the project with the static library template
        lib: bool,
        name: String,
    },
    #[structopt(name = "build")]
    Build {
        #[structopt(short = "r", long = "release")]
        /// Compiles with all optimizations
        release: bool,
    },
    #[structopt(name = "run")]
    Run {
        /// Arguments to pass to the binary on execution (use "quotes")
        arguments: Option<String>,
    },
}

fn main() {
    let options = Options::from_args();

    match options {
        Options::New{name, lib} => {
            if lib {
                Project::new(name, project::Target::Static);
            } else {
                Project::new(name, project::Target::Executable);
            }
        },
        Options::Build{release} => {

            let build = if release {
                build::build(Release::Release)
            } else {
                build::build(Release::Debug)
            };

            match build {
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                },
                _ => {},
            }
        },
        Options::Run{arguments} => {
            // Get the project file
            let project: Project;
            match Project::get(".") {
                Ok(val) => project = val,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            }

            // Unwrap the program arguments
            let program_arguments = match arguments {
                Some(v) => v,
                None => String::from(""),
            };

            // Build the program in debug mode
            match build::build(Release::Debug) {
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                },
                _ => {}
            }

            // Prevent them from being able to run the program if it is not executable
            if project.package.target != project::Target::Executable && !project.is_custom() {
                eprintln!(
"Oops!\nYour project file shows that this binary aims to be {:?}, but I can't execute those.\n{}{}",
project.package.target,
"(To be able to execute your program, please change the configuration \"target\" to equal",
" \"executable\", in your project file)\nI built the program for you anyways. :)");
                // It's real ugly, but it works ¯\_(ツ)_/¯
                return;
            } else if project.package.target == Target::Executable {
                // Execute the generated binary
                let mut child = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .arg("/C")
                        .arg(format!(".\\target\\debug\\{}.exe", project.package.name))
                        .arg(program_arguments)
                        .spawn()
                        .expect("execute built program at ./target/debug/")
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg(format!("./target/debug/{}", project.package.name))
                        .arg(program_arguments)
                        .spawn()
                        .expect("execute built program at ./target/debug/")
                };

                let status = child.wait().unwrap();
                match status.code() {
                    Some(code) => println!("Finished with code: {}", code),
                    // If, by chance, the program we executed does not return an error code,
                    // we just report that it has finished.
                    None => println!("Finished"),
                }
            }
        },
    }
}
