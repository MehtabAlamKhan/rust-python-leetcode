use std::collections::HashMap;
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let t1 = text1.chars().collect();
        let t2 = text2.chars().collect();
        return Self::lcs((text1.len() - 1) as i32, (text2.len() - 1) as i32, &t1, &t2, &mut HashMap::new())
    }
    pub fn lcs(i: i32, j: i32, t1: &Vec<char>, t2: &Vec<char>, memo: &mut HashMap<(i32, i32), i32>) -> i32{
        if i < 0 || j < 0{
            return 0;
        }
        if let Some(&cached) = memo.get(&(i, j)){
            return cached;
        }
        let mut res = 0;
        if (t1[i as usize] == t2[j as usize]){
            res = 1 + Self::lcs(i - 1, j - 1, t1, t2, memo);
        }else{
            res = Self::lcs(i, j - 1, t1, t2, memo).max(Self::lcs(i - 1, j, t1, t2, memo));
        }
        memo.insert((i,j), res);
        return res
    }
}