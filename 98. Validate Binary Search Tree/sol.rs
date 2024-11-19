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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut root = root;
        let mut q: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;
        while !q.is_empty() || root.is_some() {
            while let Some(node_rc) = root {
                prev = Some(node_rc);
                q.push(Some(node_rc.clone()));
                root = node_rc.borrow().left.clone();
            }
            if let Some(Some(node_rc)) = q.pop() {
                let node_ref = node_rc.borrow();
                if node_ref.val > prev.unwrap().borrow().val {
                    return false;
                }
                prev = Some(node_rc);
                root = node_ref.right.clone();
            }
        }
        true
    }
}
