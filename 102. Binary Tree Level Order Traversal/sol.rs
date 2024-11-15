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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut q = vec![root];
        let mut res: Vec<Vec<i32>> = vec![];
        while !q.is_empty() {
            let mut sub: Vec<i32> = vec![];
            for _ in 0..q.len() {
                if let Some(Some(node)) = q.pop() {
                    sub.push(node.borrow_mut().val);
                    if !node.borrow_mut().left.is_none() {
                        q.insert(0, node.borrow_mut().left.clone());
                    }
                    if !node.borrow_mut().right.is_none() {
                        q.insert(0, node.borrow_mut().right.clone());
                    }
                }
            }
            res.push(sub);
        }
        res
    }
}
