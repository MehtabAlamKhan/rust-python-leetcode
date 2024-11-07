impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut res = Box::new(ListNode::new(0));
        let mut tail = &mut res;
        let mut l1 = list1;
        let mut l2 = list2;

        while let (Some(ref mut n1), Some(ref mut n2)) = (l1.as_mut(), l2.as_mut()) {
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
        res.next
    }
}
