use std::collections::BinaryHeap;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
        for stone in stones {
            max_heap.push(stone);
        }
        while max_heap.len() > 1 {
            if let (Some(y), Some(x)) = (max_heap.pop(), max_heap.pop()) {
                if y != x {
                    max_heap.push(y - x);
                }
            }
        }
        if max_heap.len() > 0 {
            *max_heap.peek().unwrap()
        } else {
            0
        }
    }
}
