use crate::Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::slice_bst(&nums)
    }

    fn slice_bst(s: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        match s.len() {
            0 => None,
            1 => Some(Rc::new(RefCell::new(TreeNode::new(s[0])))),
            2 => {
                let mut node = TreeNode::new(s[1]);
                node.left = Some(Rc::new(RefCell::new(TreeNode::new(s[0]))));
                Some(Rc::new(RefCell::new(node)))
            }
            _ => {
                let mid = (s.len() as f32 / 2.0).floor() as usize;
                let mut node = TreeNode::new(s[mid]);
                node.left = Self::slice_bst(&s[..mid - 1]);
                node.right = Self::slice_bst(&s[mid + 1..]);
                Some(Rc::new(RefCell::new(node)))
            }
        }
    }
}
