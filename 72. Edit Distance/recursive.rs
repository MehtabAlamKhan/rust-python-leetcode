use std::collections::HashMap;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut memo = HashMap::new();
        let m = word1.len();
        let n = word2.len();
        let word1 = word1.chars().collect();
        let word2 = word2.chars().collect();

        return Self::helper(0, 0, &m, &n, &word1, &word2, &mut memo);
    }
    pub fn helper(
        i: usize,
        j: usize,
        m: &usize,
        n: &usize,
        word1: &Vec<char>,
        word2: &Vec<char>,
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if let Some(&cached) = memo.get(&(i, j)) {
            return cached;
        }
        if i == *m {
            return (*n - j) as i32;
        }
        if j == *n {
            return (*m - i) as i32;
        }
        let mut res = 0;
        if word1[i] == word2[j] {
            res = Self::helper(i + 1, j + 1, m, n, word1, word2, memo);
        } else {
            res = Self::helper(i + 1, j, m, n, word1, word2, memo).min(Self::helper(
                i,
                j + 1,
                m,
                n,
                word1,
                word2,
                memo,
            ));
            res = res.min(Self::helper(i + 1, j + 1, m, n, word1, word2, memo));
            res += 1;
        }
        memo.insert((i, j), res);
        return res;
    }
}
