use std::collections::HashSet;

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut res = 0;
        let (m, n) = (grid1.len(), grid1[0].len());

        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 1 && !visited.contains(&(i, j)) {
                    if Self::helper(i, j, &mut visited, &grid1, &grid2, m, n) {
                        res += 1;
                    }
                }
            }
        }
        res
    }

    fn helper(
        i: usize,
        j: usize,
        visited: &mut HashSet<(usize, usize)>,
        grid1: &Vec<Vec<i32>>,
        grid2: &Vec<Vec<i32>>,
        m: usize,
        n: usize,
    ) -> bool {
        if i >= m || j >= n || visited.contains(&(i, j)) || grid2[i][j] != 1 {
            return true;
        }

        visited.insert((i, j));

        let mut is_sub_island = grid1[i][j] == 1;
        if i > 0 {
            is_sub_island &= Self::helper(i - 1, j, visited, grid1, grid2, m, n);
        }
        if j > 0 {
            is_sub_island &= Self::helper(i, j - 1, visited, grid1, grid2, m, n);
        }
        if i + 1 < m {
            is_sub_island &= Self::helper(i + 1, j, visited, grid1, grid2, m, n);
        }
        if j + 1 < n {
            is_sub_island &= Self::helper(i, j + 1, visited, grid1, grid2, m, n);
        }
        is_sub_island
    }
}
