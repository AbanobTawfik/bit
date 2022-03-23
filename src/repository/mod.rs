mod config;
mod loader;
mod location;

use loader::Loader;

use crate::objects::{Key, Object};

struct Repository {
    loader: Loader
}

impl Repository {
    pub fn new(base_path: &str) -> Repository {
        Repository {
            loader: Loader::from(base_path).unwrap()
        }
    }

    //Follow the delta trail until a file is discovered and reconstruct the chain
    //pub fn load_file(key: &Key) 

}