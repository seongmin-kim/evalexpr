use std::slice::Iter;
use crate::Node;

/// An iterator that traverses an operator tree in pre-order.
pub struct NodeIter<'a> {
    stack: Vec<Iter<'a, Node>>,
}

impl<'a> NodeIter<'a> {
    fn new(node: &'a Node) -> Self {
        NodeIter {
            stack: vec![node.children.iter()],
        }
    }
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut pop_stack = false;
            let mut result = None;

            if let Some(last) = self.stack.last_mut() {
                if let Some(next) = last.next() {
                    result = Some(next);
                } else {
                    pop_stack = true;
                }
            } else {
                return None;
            }

            if pop_stack {
                // Can not fail because we borrowed last before.
                self.stack.pop().unwrap();
            }

            if let Some(result) = result {
                self.stack.push(result.children.iter());
                return Some(result);
            }
        }
    }
}

impl Node {
    /// Returns an iterator over all nodes in this tree.
    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        NodeIter::new(self)
    }
}
