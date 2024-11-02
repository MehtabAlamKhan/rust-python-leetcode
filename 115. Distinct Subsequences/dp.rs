impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let m = s.len();
        let n = t.len();
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut prev = vec![0; n + 1];
        prev[0] = 1;
        for i in 1..m + 1 {
            let mut cur = vec![0; n + 1];
            cur[0] = 1;
            for j in 1..n + 1 {
                if s[i - 1] == t[j - 1] {
                    cur[j] = prev[j - 1];
                }
                cur[j] += prev[j];
            }
            prev = cur;
        }
        return prev[n];
    }
}
