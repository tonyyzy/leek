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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::height(&root) != -1
    }

    fn height(r: &Option<Rc<RefCell<TreeNode>>>) -> isize {
        match r {
            Some(x) => {
                let lheight = Self::height(&x.borrow().left);
                let rheight = Self::height(&x.borrow().right);
                if (lheight - rheight).abs() > 1 || lheight == -1 || rheight == -1 {
                    -1
                } else {
                    lheight.max(rheight) + 1
                }
            }
            None => 0,
        }
    }
}
