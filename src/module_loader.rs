use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub trait FileLoader {
    fn load(&self, path: &str, base_path: Option<&Path>) -> Result<String, Box<dyn Error>>;
}

pub struct NativeFileLoader;

impl FileLoader for NativeFileLoader {
    fn load(&self, path: &str, base_path: Option<&Path>) -> Result<String, Box<dyn Error>> {

        let resolved_path = if let Some(base) = base_path {
            base.join(path)
        } else {
            Path::new(path).to_path_buf()
        };

        println!("Resolved path: {:?}", resolved_path);

        let mut file = File::open(&resolved_path)?;
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
    fn load(&self, path: &str, _base_path_: Option<&Path>) -> Result<String, Box<dyn Error>> {
        let contents = self.files.get(path);

        if contents.is_none() {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("File {} not found", path)))?;
        }

        Ok(contents.unwrap().to_string())
    }
}
