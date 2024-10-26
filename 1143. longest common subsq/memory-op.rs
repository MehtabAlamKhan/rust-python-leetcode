impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let l1 = text1.len();
        let l2 = text2.len();
        let t1:Vec<char> = text1.chars().collect();
        let t2:Vec<char> = text2.chars().collect();
        let mut dp = vec![0; l2 + 1];

        for i in 0..l1{
            let mut cur = vec![0; l2 + 1];
            for j in 0..l2{
                if t1[i] == t2[j]{
                    cur[j + 1] = 1 + dp[j];
                }else{
                    cur[j + 1] = dp[j + 1].max(cur[j]);
                }
            }
            dp = cur;
        }
        dp[l2]
    }
}