use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitData {
    author: String,
    date: String,
    message: String,
}

impl CommitData {
    pub fn new(author: String, date: String, message: String) -> CommitData {
        CommitData {
            author,
            date,
            message,
        }
    }
}
