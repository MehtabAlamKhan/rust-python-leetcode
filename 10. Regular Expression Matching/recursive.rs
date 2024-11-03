use std::collections::HashMap;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let mut memo = HashMap::new();
        let m = s.len();
        let n = p.len();
        Self::helper(0, 0, &s, &p, &mut memo, m, n)
    }

    fn helper(
        i: usize,
        j: usize,
        s: &Vec<char>,
        p: &Vec<char>,
        memo: &mut HashMap<(usize, usize), bool>,
        m: usize,
        n: usize,
    ) -> bool {
        if let Some(&cached) = memo.get(&(i, j)) {
            return cached;
        }
        if i >= m && j >= n {
            return true;
        }
        if j >= n {
            return false;
        }

        let is_match = i < m && (s[i] == p[j] || p[j] == '.');
        let res = if j + 1 < n && p[j + 1] == '*' {
            Self::helper(i, j + 2, s, p, memo, m, n)
                || (is_match && Self::helper(i + 1, j, s, p, memo, m, n))
        } else if is_match {
            Self::helper(i + 1, j + 1, s, p, memo, m, n)
        } else {
            false
        };

        memo.insert((i, j), res);
        res
    }
}
