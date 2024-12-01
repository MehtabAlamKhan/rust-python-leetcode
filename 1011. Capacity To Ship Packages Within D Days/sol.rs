impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut r: i32 = weights.iter().sum();
        let mut l: i32 = *weights.iter().max().unwrap();
        let mut res = r;
        while l <= r {
            let mid = (l + r) / 2;
            if Self::helper(mid, &weights, days) {
                res = res.min(mid);
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        res
    }
    fn helper(total_capacity: i32, weights: &Vec<i32>, days: i32) -> bool {
        let (mut ships, mut cur_cap) = (1, total_capacity);
        for w in weights {
            if cur_cap - w < 0 {
                ships += 1;
                cur_cap = total_capacity;
            }
            cur_cap -= *w;
        }
        ships <= days
    }
}
