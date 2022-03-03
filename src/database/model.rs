use std::io::Read;
use std::fs::File;
use std::path::Path;

pub struct Database {
    database_directory: String,
}

impl Database {
    pub fn new(database_directory: String) -> Database {
        Database {
            database_directory,
        }
    }

    pub fn get_data(&self, key: &String) -> Option<Vec<u8>> {
        let path = Path::new(&(self.database_directory + "/" + key));

        if path.exists() {
            let mut file = File::open(&path).unwrap();
            let mut value = Vec::new();
    
            file.read_to_end(&mut value).unwrap();

            Some(value)
        } else {
            None
        }
    }
}