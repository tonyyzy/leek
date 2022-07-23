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
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root == None {
            return false;
        }
        Self::dfs(&root, target_sum)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(x) => {
                Self::dfs(&x.borrow().left, target_sum - x.borrow().val)
                    || Self::dfs(&x.borrow().right, target_sum - x.borrow().val)
                    || (x.borrow().left == None
                        && x.borrow().right == None
                        && x.borrow().val == target_sum)
            }
            None => false,
        }
    }
}
