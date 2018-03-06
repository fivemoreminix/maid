use std::fs::{DirBuilder, File};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub package: Package,
    pub build: Option<Build>,
    pub dependencies: Option<Dependencies>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub target: Target,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Build {
    pub preferred_compiler: Option<::build::Compiler>,
    pub gnu_options: Option<Vec<String>>,
    pub clang_options: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Dependencies {
    pub header_search_directories: Option<Vec<String>>,
    pub linker_search_directories: Option<Vec<String>>,
    pub link_libraries: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Target {
    Executable,
    Static,
    Dynamic,
}

static BAD_CHARS: [char; 22] = [
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '[', ']', '{', '}', '/', '\\', ':', ';', '|',
    '<', '>', '?',
];

fn is_valid_project_name(name: &str) -> bool {
    let mut bad_chars_iter = BAD_CHARS.iter();
    let mut valid = true;
    name.chars().for_each(|c| if bad_chars_iter.any(|bad_c| c == *bad_c) {
        valid = false;
    });
    valid
}

impl Project {
    /// Creates a new project and returns its properties.
    pub fn new(name: &str) -> Result<Project, String> {
        if !is_valid_project_name(name) {
            return Err(format!("Project name may not contain any of the following restricted characters:\n{:?}", BAD_CHARS));
        }

        // Check if there is already a folder with the same name as the project
        if Path::new(format!("./{}", name).as_str()).is_dir() {
            return Err(String::from("a folder with the same name already exists."));
        }

        // Create the project directory
        let mut dir_builder = DirBuilder::new();
        dir_builder.recursive(true);
        dir_builder.create(format!("./{}/source", name)).unwrap();
        dir_builder.create(format!("./{}/include", name)).unwrap();

        // Create the template main.c source file
        let mut source_file = File::create(format!("./{}/source/main.c", name)).unwrap();
        write!(
            source_file,
            "{}",
            r#"#include <stdio.h>

int main(int argc, char *argv[])
{
    printf("Hello, world!\n");
    return 0;
}
"#
        ).unwrap();
        source_file.sync_data().unwrap();

        // Create the project file in the new folder
        let mut project_file = File::create(format!("./{}/Maid.toml", name)).unwrap();

        // Initialize the project
        let project = Project {
            package: Package {
                name: name.to_owned(),
                version: String::from("0.1.0"),
                authors: vec![String::from("Johnny Appleseed")],
                target: Target::Executable,
            },
            build: Some(Build {
                preferred_compiler: None,
                gnu_options: Some(vec![]),
                clang_options: Some(vec![]),
            }),
            dependencies: Some(Dependencies {
                header_search_directories: Some(vec![]),
                linker_search_directories: Some(vec![]),
                link_libraries: Some(vec![]),
            }),
        };

        // Serialize the project into TOML
        let toml = ::toml::to_string(&project).unwrap();

        // Write the project to the new project file
        write!(project_file, "{}", toml).unwrap();
        // Sync IO operations for the new file before continuing
        project_file.sync_all().unwrap();

        Ok(project)
    }

    /// Gets the Project in the given directory
    pub fn get(dir: &Path) -> Result<Project, &'static str> {
        // Open the project file
        let mut project_file: File;
        match File::open(dir.join("Maid.toml")) {
            Ok(val) => project_file = val,
            Err(_) => return Err("there is no Maid.toml file in the current directory."),
        }

        let mut contents = String::new();
        // Read the file into the String `contents`
        project_file.read_to_string(&mut contents).unwrap();

        // Deserialize the TOML
        let project: Project = match ::toml::from_str(contents.as_str()) {
            Ok(value) => value,
            Err(_) => panic!("The project file could not be parsed."),
        };

        Ok(project)
    }
}
