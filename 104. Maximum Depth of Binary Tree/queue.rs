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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut res = 0;
        let mut q: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(Some(node)) = q.pop() {
                    let mut bowrrowed_node = node.borrow_mut();
                    if bowrrowed_node.left.is_some() {
                        q.insert(0, bowrrowed_node.left.take());
                    }
                    if bowrrowed_node.right.is_some() {
                        q.insert(0, bowrrowed_node.right.take());
                    }
                }
            }
            res += 1;
        }
        res
    }
}
