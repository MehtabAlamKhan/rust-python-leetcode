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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut cur = root;
        let left_value = p.as_ref()?.borrow().val;
        let right_value = q.as_ref()?.borrow().val;
        while let Some(node) = cur.clone() {
            let r = node.borrow().val;
            if left_value > r && right_value > r {
                cur = node.borrow_mut().right.clone();
            } else if left_value < r && right_value < r {
                cur = node.borrow_mut().left.clone();
            } else {
                return cur;
            }
        }
        None
    }
}
