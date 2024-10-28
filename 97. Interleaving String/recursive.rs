use std::collections::HashMap;
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len(){
            return false
        }
        let str1: Vec<char> = s1.chars().collect();
        let str2: Vec<char> = s2.chars().collect();
        let str3: Vec<char> = s3.chars().collect();
        return Self::helper(&str1, &str2, &str3, 0, 0, &mut HashMap::new());
    }
    pub fn helper(s1: &Vec<char>, s2: &Vec<char>, s3: &Vec<char>, i: i32, j: i32, memo: &mut HashMap<(i32, i32), bool>)->bool{
        if let Some(&cached) = memo.get(&(i, j)){
            return cached;
        }
        if i == s1.len() as i32 && j == s2.len() as i32{
            return true;
        }
        if i < s1.len() as i32 && s1[i as usize] == s3[(i + j) as usize] && Self::helper(s1, s2, s3, i + 1, j, memo){
            return true;
        }
        if j < s2.len() as i32 && s2[j as usize] == s3[(i + j) as usize] && Self::helper(s1, s2, s3, i, j + 1, memo){
            return true;
        }
        memo.insert((i,j), false);
        return false;
    }
}