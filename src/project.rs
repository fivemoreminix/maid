use std::fs::{File, DirBuilder};
use std::io::{Write, Read};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub package: Package
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub target: Target,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Target {
    // C and C++
    Executable,
    Static,
    Dynamic,
    // Scripting
    Python,
}

impl Project {
    /// Creates a new project and returns it's properties.
    pub fn new(name: String, target: Target) -> Project {
        // Create the project directory
        let dir_builder = DirBuilder::new();
        dir_builder.create(format!("./{}", name)).unwrap();
        dir_builder.create(format!("./{}/source", name)).unwrap();
        dir_builder.create(format!("./{}/include", name)).unwrap();

        // Create the template main.c source file
        let mut source_file = File::create(format!("./{}/source/main.c", name)).unwrap();
        write!(source_file, "{}",
r#"#include <stdio.h>

int main(int argc, char **argv)
{
    printf("Hello, world!\n");
    return 0;
}
"#).unwrap();
        source_file.sync_data().unwrap();

        // Create the project file in the new folder
        let mut project_file = File::create(format!("./{}/Maid.toml", name)).unwrap();

        // Initialize the project
        let project = Project {
            package: Package {
                name: name.to_owned(),
                version: String::from("0.1.0"),
                authors: vec!(String::from("test")),
                target: target,
            }
        };

        // Serialize the project into TOML
        let toml: String = ::toml::to_string(&project).unwrap();

        // Write the project to the new project file
        write!(project_file, "{}", toml).unwrap();
        // Sync IO operations for the new file before continuing
        project_file.sync_all().unwrap();

        project
    }
    /// Gets the Project in the directory given (no "/" at the end)
    pub fn get(dir: &str) -> Result<Project, &'static str> {
        // Ensure the given directory doesn't end with a "/"
        assert!(!dir.ends_with("/"));

        // Open the project file
        let mut project_file: File;
        match File::open(format!("{}/Maid.toml", dir)) {
            Ok(val) => project_file = val,
            Err(_) => return Err("There is no Maid.toml file in the current directory."),
        }

        let mut contents = String::new();
        // Read the file into the String `contents`
        project_file.read_to_string(&mut contents).unwrap();

        // Deserialize the TOML
        let project: Project = ::toml::from_str(contents.as_str()).unwrap();

        Ok(project)
    }

    /// Returns true if this project is not using conventional build settings. (They are not using
    // target = "executable", "static", or "dynamic", in their project file)
    pub fn is_custom(&self) -> bool {
        if self.package.target == Target::Executable
        || self.package.target == Target::Static
        || self.package.target == Target::Dynamic {
            false
        } else {
            true
        }
    }
}
