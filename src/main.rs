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

#[derive(StructOpt, Debug)]
#[structopt(name = "maid", about = "A simple project manager for C, C++, and anything else.")]
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
            build::build(release);
        }
    }
}
