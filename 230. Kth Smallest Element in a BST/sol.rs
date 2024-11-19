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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut root = root;
        let mut q = vec![];
        let mut count = 0;
        let mut res: i32 = 0;
        while !q.is_empty() || root.is_some() {
            while let Some(root_ref) = root {
                q.push(Some(root_ref.clone()));
                root = root_ref.borrow().left.clone();
            }
            if let Some(Some(node_ref)) = q.pop() {
                count += 1;
                if count == k {
                    res = node_ref.borrow().val;
                }
                root = node_ref.borrow().right.clone();
            }
        }
        res
    }
}
