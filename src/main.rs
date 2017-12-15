extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod project;

use structopt::StructOpt;
use project::Project;

#[derive(StructOpt, Debug)]
#[structopt(name = "maid", about = "A simple project manager for C, C++, Assembly, and anything else.")]
enum Options {
    #[structopt(name = "new")]
    /// Creates a new project folder in the current directory
    New {
        name: String,
    },
    #[structopt(name = "test")]
    Test {

    },
}

fn main() {
    let options = Options::from_args();

    match options {
        Options::New{name} => {
            let project = Project::new(name.as_str());
        },
        Options::Test{} => {
            let project = Project::get(".");
        }
    }
}
