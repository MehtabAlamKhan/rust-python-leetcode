impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut dp = vec![0; (n + 1) as usize];
        let mut offset = 1;
        for i in 1..=n {
            if offset * 2 == i {
                offset = i;
            }
            dp[i as usize] = 1 + dp[(i - offset) as usize];
        }
        dp
    }
}
