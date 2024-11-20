use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut min_heap: BinaryHeap<Reverse<(i32, (i32, i32))>> = BinaryHeap::new();
        let mut res: Vec<Vec<i32>> = Vec::new();
        for point in points {
            if let [x, y] = point[..] {
                min_heap.push(Reverse(((x * x) + (y * y), (x, y))));
            }
        }
        let mut count = k;
        while count != 0 && min_heap.len() > 0 {
            let (x, y) = min_heap.pop().unwrap().0 .1;
            res.push(vec![x, y]);
            count -= 1;
        }
        res
    }
}
