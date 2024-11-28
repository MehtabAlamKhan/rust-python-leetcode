impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        while i < s.len() && j < t.len() {
            if s[i] == t[j] {
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }
        i == s.len()
    }
}
