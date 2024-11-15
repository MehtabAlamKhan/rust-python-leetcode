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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut q = vec![root];
        let mut res: Vec<i32> = vec![];
        while !q.is_empty() {
            for i in 0..q.len() {
                if let Some(Some(node)) = q.pop() {
                    if i == 0 {
                        res.push(node.borrow_mut().val.clone());
                    }
                    if node.borrow_mut().right.is_some() {
                        q.insert(0, node.borrow_mut().right.clone());
                    }
                    if node.borrow_mut().left.is_some() {
                        q.insert(0, node.borrow_mut().left.clone());
                    }
                }
            }
        }
        res
    }
}
