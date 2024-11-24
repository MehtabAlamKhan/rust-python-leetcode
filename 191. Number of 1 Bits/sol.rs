impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut n = n;
        let mut res = 0;
        while n > 0 {
            res += n % 2;
            n = n >> 1;
        }
        res
    }
}
