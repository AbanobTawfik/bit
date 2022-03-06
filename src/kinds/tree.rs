use serde::{Serialize, Deserialize};
use super::Key;

#[derive(Serialize, Deserialize)]
pub enum TreeDataNode {
    TreeDataDirectory {
        name: String,
        children: Vec<TreeDataNode>,
    },
    TreeDataFile {
        name: String,
        key: Key,
    },
}