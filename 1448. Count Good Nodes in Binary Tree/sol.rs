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
        let mut q = vec![(root, i32::MIN)];
        let mut res = 0;
        while let Some((node, mut max_val)) = q.pop() {
            if let Some(node_rc) = node {
                let mut node_ref = node_rc.borrow_mut();
                if node_ref.val >= max_val {
                    res += 1;
                    max_val = node_ref.val;
                }
                q.push((node_ref.left.take(), max_val));
                q.push((node_ref.right.take(), max_val));
            }
        }
        res
    }
}
