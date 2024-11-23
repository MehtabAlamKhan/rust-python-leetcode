use std::collections::{BinaryHeap, HashMap, VecDeque};
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut counts: HashMap<char, i32> = HashMap::new();
        for task in tasks {
            *counts.entry(task).or_insert(0) += 1;
        }
        for (key, value) in counts {
            max_heap.push(value);
        }
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        let mut min_time = 0;
        while !q.is_empty() || !max_heap.is_empty() {
            min_time += 1;
            if let Some(task) = max_heap.pop() {
                if task - 1 > 0 {
                    q.push_front((task - 1, min_time + n));
                }
            }
            if !q.is_empty() && min_time == q.back().unwrap().1 {
                max_heap.push(q.pop_back().unwrap().0);
            }
        }
        min_time
    }
}
