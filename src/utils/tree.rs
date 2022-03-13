use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Node<T>
where
    T: Eq + Clone 
{
    value: T,
    children: Vec<Node<T>>
}

#[derive(Serialize, Deserialize)]
pub struct Tree<T> 
where
    T: Eq + Clone 
{
    root: Option<Node<T>>
}

impl<T> Tree<T> 
where
    T: Eq + Clone 
{
    pub fn new() -> Tree<T> {
        Tree { root: None }
    }



    pub fn len(&self) -> usize {
        if let Some(node) = &self.root {
            let mut node_stack = vec![node];
            let mut count = 1;

            while node_stack.len() > 0 {
                let curr_node = node_stack.pop().unwrap();

                count += curr_node.children.len();

                node_stack.extend(&curr_node.children);
            }

            count
        } else {
            0
        }
    }
}