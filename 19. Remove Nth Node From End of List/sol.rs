// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut right = head.as_ref();
        let mut count = n;
        while count > 0 {
            if let Some(node) = right {
                count -= 1;
                right = node.next.as_ref();
            } else {
                break;
            }
        }
        let mut res = Some(Box::new(ListNode {
            val: 0,
            next: head.clone(),
        }));
        let mut left = &mut res;

        while let Some(node) = right {
            left = &mut left.as_mut().unwrap().next;
            right = node.next.as_ref();
        }

        if let Some(first) = left {
            if let Some(second) = first.next.take() {
                first.next = second.next;
            }
        }
        res.unwrap().next
    }
}
