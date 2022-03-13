mod config;
mod loader;
mod location;

use loader::Loader;

use crate::objects::{Key, Object};

struct Repository<'a> {
    loader: Loader<'a>
}

impl<'a> Repository<'a> {
    pub fn new(base_path: &str) -> Repository {
        Repository {
            loader: Loader::from(base_path).unwrap()
        }
    }

    pub fn load_object(&mut self, key: &Key) -> Option<Object<u8>> {
        if self.loader.check_data_exists(key) {
            Some(self.loader.load_data(key).unwrap())
        } else {
            None
        }
    }


}