use std::collections::BinaryHeap;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
        for n in nums {
            max_heap.push(n);
        }
        let mut count = 1;

        while count != k {
            max_heap.pop();
            count += 1;
        }
        max_heap.pop().unwrap()
    }
}
