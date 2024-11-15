// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() {
            return false;
        }
        if Self::helper(root.clone(), sub_root.clone()) {
            return true;
        }
        if let Some(root_node) = root {
            return Self::is_subtree(root_node.borrow().left.clone(), sub_root.clone())
                || Self::is_subtree(root_node.borrow().right.clone(), sub_root.clone());
        }
        false
    }
    pub fn helper(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }

        if let (Some(left), Some(right)) = (p, q) {
            if left.borrow().val != right.borrow().val {
                return false;
            }
            return Self::helper(left.borrow().left.clone(), right.borrow().left.clone())
                && Self::helper(left.borrow().right.clone(), right.borrow().right.clone());
        }
        false
    }
}
