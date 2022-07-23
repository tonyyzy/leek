use crate::Solution;
use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut out = vec![];
        if let Some(r) = root {
            out.append(&mut Solution::inorder_traversal(r.borrow().left.clone()));
            out.push(r.borrow().val);
            out.append(&mut Solution::inorder_traversal(r.borrow().right.clone()));
        };
        out
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;

    use super::Solution;
    const NULL: i32 = i32::MIN;

    #[test]
    fn test() {
        assert_eq!(
            Solution::inorder_traversal(TreeNode::from_list(&[1, NULL, 4, 5])),
            vec![1, 5, 4]
        );
        assert_eq!(
            Solution::inorder_traversal(TreeNode::from_list(&[1, 2, 3, 4, 5])),
            vec![4, 2, 5, 1, 3]
        );
    }
}
