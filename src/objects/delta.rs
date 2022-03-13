use core::hash::Hash;
use serde::{Deserialize, Serialize};
use similar::{capture_diff_slices, Algorithm, DiffOp};
use std::vec::Vec;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum DeltaDataOp<T>
where
    T: Eq + Hash + Ord + Clone,
{
    Delete { pos: usize, length: usize },
    Insert { pos: usize, value: Vec<T> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaData<T>
where
    T: Eq + Hash + Ord + Clone,
{
    deltas: Vec<DeltaDataOp<T>>,
}

impl<T> DeltaData<T>
where
    T: Eq + Hash + Ord + Clone,
{
    pub fn new(old: &Vec<T>, new: &Vec<T>) -> DeltaData<T> {
        let diffs = capture_diff_slices(Algorithm::Myers, old, new);

        let mut deltas = Vec::<DeltaDataOp<T>>::new();

        for diff in diffs.into_iter() {
            match diff {
                DiffOp::Equal { .. } => {}
                DiffOp::Delete {
                    old_index, old_len, ..
                } => {
                    deltas.push(DeltaDataOp::Delete {
                        pos: old_index,
                        length: old_len,
                    });
                }
                DiffOp::Insert {
                    old_index,
                    new_index,
                    new_len,
                } => {
                    deltas.push(DeltaDataOp::Insert {
                        pos: old_index,
                        value: new[new_index..new_index + new_len].to_vec(),
                    });
                }
                DiffOp::Replace {
                    old_index,
                    old_len,
                    new_index,
                    new_len,
                } => {
                    deltas.push(DeltaDataOp::Delete {
                        pos: old_index,
                        length: old_len,
                    });
                    deltas.push(DeltaDataOp::Insert {
                        pos: old_index,
                        value: new[new_index..new_index + new_len].to_vec(),
                    });
                }
            };
        }

        DeltaData { deltas }
    }

    pub fn apply(&self, old: &Vec<T>) -> Vec<T> {
        let mut out = old.clone();

        for delta in &self.deltas {
            match delta {
                DeltaDataOp::Delete { pos, length } => {
                    out.drain(*pos..*pos + *length);
                }
                DeltaDataOp::Insert { pos, value } => {
                    out.splice(*pos..*pos + value.len(), value.clone());
                }
            }
        }

        out
    }
}
