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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        //need two lists for fast and slow. since one will be as mutable
        //thats why cant use single list for both pointers.
        let mut res = head.clone();

        //two pointers for fast and slow
        let mut fast = head.as_ref();
        let mut slow = res.as_mut();

        while let Some(fast_node) = fast {
            if fast_node.next.is_none() || fast_node.next.as_ref().unwrap().next.is_none() {
                break;
            }
            slow = slow.unwrap().next.as_mut();
            fast = fast_node.next.as_ref().unwrap().next.as_ref();
        }

        //extract the second half from slow and delink it
        //from first half using take
        let mut second_half = slow.unwrap().as_mut().next.take();

        //reverse the second half
        let mut prev = None;
        while let Some(mut node) = second_half {
            second_half = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        //reverse second half = prev
        //merge both
        let mut res_traverse = res.as_mut();
        while let (Some(first), Some(mut second)) = (res_traverse, prev) {
            let first_next = first.next.take();
            let second_next = second.next.take();

            //update res_traverse
            first.next = Some(second);
            first.next.as_mut().unwrap().next = first_next;

            //update both halves
            res_traverse = first.next.as_mut().unwrap().next.as_mut();
            prev = second_next;
        }
        *head = res;
    }
}
