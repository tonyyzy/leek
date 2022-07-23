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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        Self::dfs113(&root, target_sum)
    }

    fn dfs113(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        match root {
            Some(x) => {
                if x.borrow().left == None && x.borrow().right == None {
                    if x.borrow().val == target_sum {
                        ret.push(vec![target_sum]);
                    }
                }
                let mut lsearch = Self::dfs113(&x.borrow().left, target_sum - x.borrow().val);
                let mut rsearch = Self::dfs113(&x.borrow().right, target_sum - x.borrow().val);
                for i in lsearch.iter_mut() {
                    let mut tmp = vec![x.borrow().val];
                    tmp.append(i);
                    ret.push(tmp)
                }
                for i in rsearch.iter_mut() {
                    let mut tmp = vec![x.borrow().val];
                    tmp.append(i);
                    ret.push(tmp)
                }
                ret
            }
            None => ret,
        }
    }
}
