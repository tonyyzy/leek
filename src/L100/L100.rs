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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        } else if p.is_none() || q.is_none() {
            return false;
        }
        let mut pv = vec![p];
        let mut qv = vec![q];
        while pv == qv {
            let mut new_pv = vec![];
            let mut new_qv = vec![];
            for n in &pv {
                if let Some(tn) = n {
                    new_pv.push(tn.borrow().left.clone());
                    new_pv.push(tn.borrow().right.clone());
                }
            }
            for n in &qv {
                if let Some(tn) = n {
                    new_qv.push(tn.borrow().left.clone());
                    new_qv.push(tn.borrow().right.clone());
                }
            }

            if new_pv.is_empty() && new_qv.is_empty() {
                return true;
            }
            pv = new_pv;
            qv = new_qv;
        }
        false
    }
}
