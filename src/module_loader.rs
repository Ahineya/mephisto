use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub trait FileLoader {
    fn load(&self, path: &str) -> Result<String, Box<dyn Error>>;
}

pub struct NativeFileLoader;

impl FileLoader for NativeFileLoader {
    fn load(&self, path: &str) -> Result<String, Box<dyn Error>> {
        let current_dir = std::env::current_dir()?;

        println!("Current dir: {:?}", current_dir);

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

pub struct StubFileLoader {
    pub files: HashMap<String, String>,
}

impl StubFileLoader {
    pub fn new(files: HashMap<String, String>) -> StubFileLoader {
        StubFileLoader {
            files,
        }
    }
}
impl FileLoader for StubFileLoader {
    fn load(&self, path: &str) -> Result<String, Box<dyn Error>> {
        let contents = self.files.get(path);

        if contents.is_none() {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("File {} not found", path)))?;
        }

        Ok(contents.unwrap().to_string())
    }
}
