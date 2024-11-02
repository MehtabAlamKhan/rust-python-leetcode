use std::collections::HashMap;
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s1 = s.chars().collect();
        let s2 = t.chars().collect();
        let mut memo = HashMap::new();
        return Self::helper(0, 0, &s1, &s2, &mut memo);
    }
    pub fn helper(
        i: usize,
        j: usize,
        s1: &Vec<char>,
        s2: &Vec<char>,
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if let Some(&cached) = memo.get(&(i, j)) {
            return cached;
        }
        if j == s2.len() {
            return 1;
        }
        if i >= s1.len() || j >= s2.len() {
            return 0;
        }
        let mut res = 0;
        if (s1[i] == s2[j]) {
            res += Self::helper(i + 1, j + 1, s1, s2, memo);
        }
        res += Self::helper(i + 1, j, s1, s2, memo);
        memo.insert((i, j), res);
        return res;
    }
}
