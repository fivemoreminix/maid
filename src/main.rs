extern crate ansi_term;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate toml;

mod build;
mod project;
mod utils;
mod user;

use structopt::StructOpt;
use project::Project;
use std::path::Path;
use ansi_term::Color::Green;

#[derive(StructOpt)]
#[structopt(name = "maid", about = "A modern project manager for C and C++.")]
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
    #[structopt(name = "clean")]
    Clean,
}

fn main() {
    let options = Options::from_args();

    std::panic::set_hook(Box::new(|panic_info| {
        match panic_info.payload().downcast_ref::<&str>() {
            Some(message) => eprintln!("maid: error: {}", message),
            None => eprintln!("maid: error, exiting"),
        }
    }));

    // Enable color support
    ansi_term::enable_ansi_support().unwrap();

    match options {
        Options::New { name, lib } => {
            if lib {
                Project::new(name).unwrap();
            } else {
                Project::new(name).unwrap();
            }
        }
        Options::Build { verbose, release } => build::build(release, verbose).unwrap(),
        Options::Run { arguments } => {
            // Get the project file
            let project = Project::get(Path::new(".")).unwrap();

            // Unwrap the program arguments
            let arguments = match arguments {
                Some(v) => v,
                None => String::from(""),
            };

            // Build the program in debug mode, without verbosity
            build::build(false, false).unwrap();

            if project.package.target != project::Target::Executable {
                // Prevent them from being able to run the program if it is not executable
                panic!("Can't execute {:?} targets.", project.package.target);
            } else {
                println!("     {} `{}`", Green.paint("Running"), project.package.name);

                // Execute the generated binary
                let result = if cfg!(target_os = "windows") {
                    utils::shell_command(
                        &format!("./target/debug/{}.exe {}", project.package.name, arguments),
                        false,
                        false,
                    )
                } else {
                    utils::shell_command(
                        &format!("./target/debug/{} {}", project.package.name, arguments),
                        false,
                        false,
                    )
                };

                match result.unwrap().code() {
                    Some(code) => if code != 0 {
                        println!("Exited with code: {}", code)
                    },
                    None => {}
                }
            }
        }
        Options::Clean => match Project::get(Path::new(".")) {
            Ok(_) => std::fs::remove_dir_all("./target").unwrap(),
            Err(_) => panic!("Project folder not within current directory."),
        },
    }
}
