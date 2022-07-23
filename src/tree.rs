use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

const NULL: i32 = i32::MIN;

#[derive(Debug, PartialEq)]
enum Side {
    Left,
    Right,
}

impl Side {
    fn next(&mut self) {
        if *self == Side::Left {
            *self = Side::Right
        } else {
            *self = Side::Left
        }
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_list(list: &[i32]) -> Option<Rc<RefCell<Self>>> {
        let mut list_iter = list.iter();
        let root;
        if let Some(x) = list_iter.next() {
            if *x != i32::MIN {
                root = Rc::new(RefCell::new(TreeNode::new(*x)));
            } else {
                return None;
            }
        } else {
            return None;
        };

        let mut cursors = vec![root.clone()];
        let mut next_cursors = vec![];
        let mut pointer = Side::Left;
        let mut cursor = cursors.pop().unwrap();

        while let Some(x) = list_iter.next() {
            if *x != i32::MIN {
                let new_node = Rc::new(RefCell::new(TreeNode::new(*x)));
                next_cursors.push(new_node.clone());
                if pointer == Side::Left {
                    cursor.borrow_mut().left = Some(new_node.clone());
                    pointer.next();
                } else {
                    cursor.borrow_mut().right = Some(new_node.clone());
                    pointer.next();
                    if let Some(node) = cursors.pop() {
                        cursor = node.clone();
                    } else {
                        next_cursors.reverse();
                        std::mem::swap(&mut cursors, &mut next_cursors);
                        cursor = cursors.pop().unwrap();
                    }
                }
            } else {
                pointer.next()
            }
        }
        Some(root)
    }

    fn breadth_frist(
        root: Option<Rc<RefCell<Self>>>,
    ) -> Vec<Rc<RefCell<Self>>> {
        let mut out = vec![];
        if let Some(r) = root {
            let mut node_list = vec![r];
            let mut new_node_list = vec![];
            while node_list.len() > 0 {
                for node in node_list.iter() {
                    out.push(node.clone());
                    if let Some(x) = node.borrow_mut().left.clone() {
                        new_node_list.push(x)
                    }
                    if let Some(x) = node.borrow_mut().right.clone() {
                        new_node_list.push(x)
                    }
                }
                node_list = new_node_list;
                new_node_list = vec![];
            }
        };
        out
    }

    fn inorder(root: Option<Rc<RefCell<Self>>>) -> Vec<i32> {
        let mut out = vec![];
        if let Some(r) = root {
            out.append(&mut TreeNode::inorder(r.borrow().left.clone()));
            out.push(r.borrow().val);
            out.append(&mut TreeNode::inorder(r.borrow().right.clone()));
        };
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_list() {
        println!(
            "{:?}",
            TreeNode::from_list(&[1, NULL, 4, 5]).unwrap().borrow_mut()
        );
        assert_eq!(TreeNode::from_list(&[NULL]), None);
        assert_eq!(TreeNode::from_list(&[]), None);
        for k in TreeNode::breadth_frist(TreeNode::from_list(&[
            1, NULL, 2, 3, 4, 5, NULL, 6, 7, 8, NULL
        ])) {
            println!("{}", k.borrow().val)
        }
    }
}
