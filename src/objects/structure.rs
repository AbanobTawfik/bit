use super::Key;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct StructureEntry {
    key: Key,
    name: String,
}
