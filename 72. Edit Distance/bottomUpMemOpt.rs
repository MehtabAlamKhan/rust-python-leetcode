impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();
        let mut dp: Vec<i32> = (0..=m as i32).collect();
        for i in 1..n + 1 {
            let mut cur: Vec<i32> = vec![0; m + 1];
            cur[0] = i as i32;
            for j in 1..m + 1 {
                if word1[j - 1] == word2[i - 1] {
                    cur[j] = dp[j - 1];
                } else {
                    cur[j] = dp[j - 1].min(dp[j]).min(cur[j - 1]) + 1;
                }
            }
            dp = cur
        }
        return dp[m];
    }
}
