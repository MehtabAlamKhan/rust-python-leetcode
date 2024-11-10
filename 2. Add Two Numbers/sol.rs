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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut ptr1 = &l1;
        let mut ptr2 = &l2;
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: None }));
        let mut cur = &mut dummy_head;

        while ptr1.is_some() || ptr2.is_some() {
            let mut sum = carry;
            if let Some(first) = ptr1 {
                sum += first.val;
                ptr1 = &first.next;
            }
            if let Some(second) = ptr2 {
                sum += second.val;
                ptr2 = &second.next;
            }
            carry = sum / 10;
            sum %= 10;

            cur.as_mut().unwrap().next = Some(Box::new(ListNode {
                val: sum,
                next: None,
            }));
            cur = &mut cur.as_mut().unwrap().next;
        }
        if carry > 0 {
            cur.as_mut().unwrap().next = Some(Box::new(ListNode {
                val: carry,
                next: None,
            }));
        }
        dummy_head.unwrap().next
    }
}
