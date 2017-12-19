extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod build;
mod project;
mod utils;
mod user;

use structopt::StructOpt;
use project::{Project, Target};

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
        #[structopt(short = "v", long = "verbose")]
        /// Gives you more information as to what is happening
        verbose: bool,

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
                match Project::new(name, project::Target::Static) {
                    Err(e) => eprintln!("{}", e),
                    _ => {},
                }
            } else {
                match Project::new(name, project::Target::Executable) {
                    Err(e) => eprintln!("{}", e),
                    _ => {},
                }
            }
        },
        Options::Build{verbose, release} => {
            match build::build(release, verbose) {
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
                },
            }

            // Unwrap the program arguments
            let arguments = match arguments {
                Some(v) => v,
                None => String::from(""),
            };

            // Build the program in debug mode, without verbosity
            match build::build(false, false) {
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                },
                _ => {},
            }

            // Prevent them from being able to run the program if it is not executable
            if project.package.target != project::Target::Executable || project.is_custom() {
                println!(
"Oops!\nYour project file shows that this binary aims to be {:?}, but I can't execute those.\n{}{}",
project.package.target,
"(To be able to execute your program, please change the configuration \"target\" to equal",
" \"executable\", in your project file)\nI built the program for you anyways. :)");
                // It's real ugly, but it works ¯\_(ツ)_/¯
                return;
            } else if project.package.target == Target::Executable {
                // Execute the generated binary
                let child = utils::shell_command(format!("./target/debug/{}.exe {}", project.package.name, arguments))
                    .expect("execute built program in ./target/debug/");

                match child.code() {
                    Some(code) => if code != 0 {
                        println!("Exited with code: {}", code)
                    },
                    None => {},
                }
            }
        },
    }
}
