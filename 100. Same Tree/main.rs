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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if let (Some(p_node), Some(q_node)) = (p, q) {
            if p_node.borrow().val != q_node.borrow().val {
                return false;
            }
            return Self::is_same_tree(p_node.borrow().left.clone(), q_node.borrow().left.clone())
                && Self::is_same_tree(
                    p_node.borrow().right.clone(),
                    q_node.borrow().right.clone(),
                );
        } else {
            return false;
        }
    }
}
