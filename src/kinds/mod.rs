mod commit;
mod delta;
mod tree;

use self::{commit::CommitData, delta::DeltaData, tree::TreeDataNode};

use core::hash::Hash;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::fmt;
use std::vec::Vec;

pub type Key = String;

//This is what is stored in the db so include any info you need here
//T defines the base type that files are formatted in
//Good choices might be String or u8
//
//TODO add support for different base types
#[derive(Serialize, Deserialize)]
pub enum Kind<T>
where
    T: Eq + Hash + Ord + Clone,
{
    File { data: Vec<T> },
    Delta { data: DeltaData<T>, base: Key },
    Tree { data_root: TreeDataNode },
    Commit { data: CommitData },
}
