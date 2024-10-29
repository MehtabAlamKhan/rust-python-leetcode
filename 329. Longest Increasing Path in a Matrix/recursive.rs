use std::collections::HashMap;
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut res = 0;
        let mut memo = HashMap::new();
        for r in 0..rows {
            for c in 0..cols {
                res = res.max(Self::helper(r, c, rows, cols, -1, &matrix, &mut memo));
            }
        }
        return res;
    }
    pub fn helper(
        r: usize,
        c: usize,
        rows: usize,
        cols: usize,
        prev: i32,
        matrix: &[Vec<i32>],
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if r >= rows || c >= cols || matrix[r][c] <= prev {
            return 0;
        }
        if let Some(&cached) = memo.get(&(r, c)) {
            return cached;
        }
        let mut res = 1;
        if r + 1 < rows {
            res = res.max(1 + Self::helper(r + 1, c, rows, cols, matrix[r][c], matrix, memo))
        }
        if r > 0 {
            res = res.max(1 + Self::helper(r - 1, c, rows, cols, matrix[r][c], matrix, memo))
        }
        if c + 1 < cols {
            res = res.max(1 + Self::helper(r, c + 1, rows, cols, matrix[r][c], matrix, memo))
        }
        if c > 0 {
            res = res.max(1 + Self::helper(r, c - 1, rows, cols, matrix[r][c], matrix, memo))
        }

        memo.insert((r, c), res);
        return res;
    }
}
