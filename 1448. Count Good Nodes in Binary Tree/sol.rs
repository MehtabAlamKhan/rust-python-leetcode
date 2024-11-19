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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q: Vec<(Rc<RefCell<TreeNode>>, i32)> =
            vec![(root.clone().unwrap(), root.unwrap().borrow().val)];
        let mut res = 0;
        while let Some((node, max_val)) = q.pop() {
            if node.borrow().val >= max_val {
                res += 1;
            }
            if node.borrow().left.is_some() {
                q.insert(
                    0,
                    (
                        node.borrow().left.clone().unwrap(),
                        node.borrow().val.max(max_val),
                    ),
                );
            }
            if node.borrow().right.is_some() {
                q.insert(
                    0,
                    (
                        node.borrow().right.clone().unwrap(),
                        node.borrow().val.max(max_val),
                    ),
                );
            }
        }
        res
    }
}
