extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod build;
mod project;

use structopt::StructOpt;
use project::Project;
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
        #[structopt(long = "release")]
        /// Compiles with optimizations
        release: bool,
    },
    #[structopt(name = "run")]
    Run {
        program_arguments: Option<String>,
    },
}

fn main() {
    let options = Options::from_args();

    match options {
        Options::New{name, lib} => {
            if lib {
                Project::new(name.as_str(), String::from("static"));
            } else {
                Project::new(name.as_str(), String::from("executable"));
            }
        },
        Options::Build{release} => {
            match build::build(release) {
                Err(e) => {
                    println!("{}", e);
                    return;
                },
                _ => {},
            }
        },
        Options::Run{program_arguments} => {
            // Get the project file
            let project: Project;
            match Project::get(".") {
                Ok(val) => project = val,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            }
            // Unwrap the program arguments
            let arguments = match program_arguments {
                Some(v) => v,
                None => String::from(""),
            };
            // Build the program in debug mode
            build::build(false).unwrap();
            // Execute the generated binary
            let mut child = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .arg("/C")
                    .arg(format!(".\\target\\debug\\{}.exe", project.package.name))
                    .arg(arguments)
                    .spawn()
                    .expect("execute built program at ./target/debug/")
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(format!("./target/debug/{}", project.package.name))
                    .arg(arguments)
                    .spawn()
                    .expect("execute built program at ./target/debug/")
            };

            let status = child.wait().unwrap();
            match status.code() {
                Some(code) => println!("Finished with code: {}", code),
                None => println!("Finished"),
            }
        },
    }
}
