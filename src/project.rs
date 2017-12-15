use std::fs::{File, DirBuilder};
use std::io::{Write, Read};

#[derive(Debug)]
pub struct Project {
    name: String,
}

impl Project {
    /// Creates a new project and returns it's properties.
    pub fn new(name: &str) -> Project {
        // Create the project directory
        DirBuilder::new().create(format!("./{}", name)).unwrap();
        // Create the project file in the new folder
        let mut project_file = File::create(format!("./{}/maid.toml", name)).unwrap();
        // Initialize the project file with the defaults
        write!(project_file, "[package]\nname = \"{}\"\nversion = \"0.1.0\"\n", name).unwrap();
        // Sync IO operations for the new file before continuing
        project_file.sync_all().unwrap();

        Project { name: name.to_owned() }
    }
    /// Gets the Project in the directory given (no "/" at the end)
    pub fn get(dir: &str) -> Project {
        assert!(!dir.ends_with("/")); // Ensure the given directory doesn't end with a "/"
        // Open the project file
        let mut project_file = File::open(format!("{}/maid.toml", dir)).unwrap();

        let mut contents = String::new();
        // Read the file into the String `contents`
        project_file.read_to_string(&mut contents).unwrap();
        println!("{:?}", contents);
        
        Project { name: "".to_owned() }
    }
}
