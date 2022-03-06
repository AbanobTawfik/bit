mod object;
mod file_manager;

use std::path::Path;
use self::object::Database;

use crate::kinds::Key;

struct Repository {
    raw_path: String,
    database: Database,
}

impl Repository {
    pub fn new(raw_path: &String) -> Repository {
        Repository {
            raw_path: raw_path.clone(),
            database: Database::new("db".to_string())
        }
    }

    pub fn load_object(&self, key: &Key) -> Option<Vec<u8>> {
        self.database.load_data(&self.raw_path, &key)
    }


}