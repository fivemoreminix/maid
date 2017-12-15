extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate toml;

mod project;

use project::Project;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "maid", about = "The easy, modern build tool for C, C++, and Assembly.")]
enum Options {
    #[structopt(name = "new")]
    /// Create a new project folder
    New {
        name: String,
    },
    #[structopt(name = "test")]
    Test {

    },
}

fn main() {
    let options = Options::from_args();
    println!("{:?}", options);

    match options {
        Options::New{name} => {
            let project = Project::new(name.as_str());
        },
        Options::Test{} => {
            let project = Project::get(".");
        }
    }
}
