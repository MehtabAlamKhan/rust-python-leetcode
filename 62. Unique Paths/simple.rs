/// time - O(m * n) 
/// space - O(n+m)
use std::collections::HashMap;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        return Self::gridTraveler(&m, &n, 0, 0, &mut HashMap::new())
    }
    pub fn gridTraveler(m: &i32, n: &i32, i: i32, j: i32, memo: &mut HashMap<(i32, i32), i32>)->i32{
        if let Some(&cached) = memo.get(&(i,j)){
            return cached
        }
        if i == *m - 1 && j == *n - 1{
            return 1;
        }
        if i >= *m || j >= *n {
            return 0;
        }
        let paths = Self::gridTraveler(m, n, i + 1, j, memo) + Self::gridTraveler(m, n, i, j + 1, memo);
        memo.insert((i,j), paths);
        return paths
    }
}
