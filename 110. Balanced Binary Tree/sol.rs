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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut isBal = true;
        Self::helper(&mut isBal, root);
        isBal
    }
    pub fn helper(isBal: &mut bool, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root_node = root.as_ref().unwrap();
        let left = Self::helper(isBal, root_node.borrow().left.clone());
        let right = Self::helper(isBal, root_node.borrow().right.clone());
        if (left - right).abs() > 1 {
            *isBal = false;
        }
        1 + left.max(right)
    }
}
