use std::{collections::HashMap, fs::File, path::Path};
use serde::Deserialize;
use rmp_serde::decode;


use crate::kinds::Key;

enum Entry {
    Directory {
        children: Vec<String>,
    },
    FileSet,
    File {
        init: String,
    },
}

#[derive(PartialEq, Eq, Hash)]
enum CoreKind {
    Config,
    Database,
    Dictionary,
}

struct FileManager {
    raw_root_path: String,
    entries: HashMap<String, Entry>,
    core_locations: HashMap<CoreKind, String>
}

fn load_file(raw_path)

impl FileManager {
    fn new(raw_root_path: String) -> FileManager {
        FileManager {
            raw_root_path,
            entries: HashMap::from([
                ("config".to_string(), Entry::File {
                    init: "".to_string()
                }),
                ("db".to_string(), Entry::Directory {
                    children: vec!["obj".to_string()]
                }),
                (("obj").to_string(), Entry::FileSet),
                ("dict".to_string(), Entry::File {
                    init: "".to_string()
                }),
            ]),
            core_locations: HashMap::from([
                (CoreKind::Config, "config".to_string()),
                (CoreKind::Database, "db".to_string()),
                (CoreKind::Dictionary, "dict".to_string()),
            ]),
        }
    }

    fn load_config(&self) {

    }

    fn load_dictionary(&self) {
        let raw_path = format!("{}/{}", self.raw_root_path, self.core_locations[&CoreKind::Dictionary]);
        let path = Path::new(&raw_path);

        if path.exists()
    }

    fn load_database_object(&self, key: &Key) -> Option<File>{
        let raw_path = format!("{}/{}/{}", self.raw_root_path, self.core_locations[&CoreKind::Database], key);
        let path = Path::new(&raw_path);

        if path.exists() {
            let file = File::open(&path);



            Some(File::open(&path))
        } else {
            None
        }
    }
}