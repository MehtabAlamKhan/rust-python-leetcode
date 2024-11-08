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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut res = Box::new(ListNode::new(0));
        let mut tail = &mut res;
        let mut l1 = list1;
        let mut l2 = list2;

        while let (Some(mut n1), Some(mut n2)) = (l1.clone(), l2.clone()) {
            if n1.val < n2.val {
                l1 = n1.next.take();
                tail.next = Some(n1.clone());
            } else {
                l2 = n2.next.take();
                tail.next = Some(n2.clone());
            }
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = if l1.is_some() { l1 } else { l2 };
        return res.next;
    }
}
