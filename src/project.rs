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

fn is_valid_project_name(name: &str) -> bool {
    name.chars().all(|c| match c {
        'a' ... 'z' |
        'A' ... 'Z' |
        '0' ... '9' |
        '_' | '-' => true,
        _ => false,
    })
}

impl Project {
    /// Creates a new project and returns its properties.
    pub fn new(name: &str) -> Result<Self, ProjectError> {
        if !is_valid_project_name(name) {
            return Err(ProjectError { error_type: ProjectErrorType::ProjectNameContainsInvalidCharacters, description: String::from("Project name must match the regex: (a-zA-Z)+") });
        }

        // Check if there is already a folder with the same name as the project
        if Path::new(&format!("./{}", name)).is_dir() {
            return Err(ProjectError { error_type: ProjectErrorType::ProjectWithSameNameAlreadyExists, description: String::from("A project folder with the same name already exists within the current directory.") });
        }

        // Create the project directory
        let mut dir_builder = DirBuilder::new();
        dir_builder.recursive(true);
        dir_builder.create(format!("./{}/source", name)).unwrap();
        dir_builder.create(format!("./{}/include", name)).unwrap();

        // Create the template main.c source file
        let mut source_file = File::create(format!("./{}/source/main.c", name)).unwrap();
        source_file
            .write_all(
                r#"#include <stdio.h>

int main(int argc, char *argv[])
{
    printf("Hello, world!\n");
    return 0;
}
"#.as_bytes(),
            )
            .unwrap();
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

        project_file.write_all(toml.as_bytes()).unwrap();
        // Sync IO operations for the new file before continuing
        project_file.sync_all().unwrap();

        Ok(project)
    }

    /// Gets the Project in the given directory
    pub fn get() -> Result<Self, ProjectError> {
        // Open the project file
        let mut project_file = match File::open("./Maid.toml") {
            Ok(val) => val,
            Err(_) => match File::open("../Maid.toml") {
                Ok(val) => val,
                Err(_) => {
                    return Err(ProjectError {
                        error_type: ProjectErrorType::MaidFileNotFound,
                        description: String::from("No Maid.toml in the current directory."),
                    })
                }
            },
        };

        let mut contents = String::new();
        project_file.read_to_string(&mut contents).unwrap();

        // Deserialize the TOML
        let project: Project = match ::toml::from_str(&contents) {
            Ok(value) => value,
            Err(_) => {
                return Err(ProjectError {
                    error_type: ProjectErrorType::ProjectFileCouldNotBeParsed,
                    description: String::from("The project file could not be parsed."),
                })
            }
        };

        if is_valid_project_name(&project.package.name) {
            Ok(project)
        } else {
            Err(ProjectError { error_type: ProjectErrorType::ProjectNameContainsInvalidCharacters, description: String::from("Project name must match the regex: (a-zA-Z)+") })
        }
    }
}

#[derive(Debug)]
pub enum ProjectErrorType {
    MaidFileNotFound,
    ProjectFileCouldNotBeParsed,
    ProjectNameContainsInvalidCharacters,
    ProjectWithSameNameAlreadyExists,
}

#[derive(Debug)]
pub struct ProjectError {
    pub error_type: ProjectErrorType,
    pub description: String,
}

impl ::std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{} ({:?})", self.description, self.error_type)
    }
}
