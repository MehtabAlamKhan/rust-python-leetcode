use std::collections::HashMap;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let mut memo = HashMap::new();
        let m = s.len();
        let n = p.len();
        Self::helper(0, 0, &s, &p, &mut memo, &m, &n)
    }
    pub fn helper(
        i: usize,
        j: usize,
        s: &Vec<char>,
        p: &Vec<char>,
        memo: &mut HashMap<(usize, usize), bool>,
        m: &usize,
        n: &usize,
    ) -> bool {
        if let Some(&cached) = memo.get(&(i, j)) {
            return cached;
        }
        if i >= *m && j >= *n {
            return true;
        }
        if j >= *n {
            return false;
        }
        let mut res = false;
        let mactchh = i < *m && (s[i] == p[j] || p[j] == '.');
        if j + 1 < *n && p[j + 1] == '*' {
            res = Self::helper(i, j + 2, s, p, memo, m, n)
                || (mactchh && Self::helper(i + 1, j, s, p, memo, m, n));
            memo.insert((i, j), res);
            return res;
        }
        if mactchh {
            res = Self::helper(i + 1, j + 1, s, p, memo, m, n);
            memo.insert((i, j), res);
            return res;
        }
        memo.insert((i, j), false);
        return false;
    }
}
