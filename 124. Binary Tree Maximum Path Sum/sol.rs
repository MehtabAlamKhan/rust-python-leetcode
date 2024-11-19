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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = i32::MIN;
        Self::helper(&mut res, root);
        res
    }
    pub fn helper(res: &mut i32, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node_rc) = root {
            let left = (0).max(Self::helper(res, node_rc.borrow().left.clone()));
            let right = (0).max(Self::helper(res, node_rc.borrow().right.clone()));
            *res = (*res).max(node_rc.borrow().val + left + right);
            node_rc.borrow().val + left.max(right)
        } else {
            0
        }
    }
}
