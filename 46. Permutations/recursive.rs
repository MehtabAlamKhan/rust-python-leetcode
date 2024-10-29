impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut perms = Vec::new();
        let mut used = vec![false; nums.len()];
        Self::helper(&nums, &mut perms, &mut used, &mut res);
        return res
    }
    pub fn helper(nums: &Vec<i32>, perms: &mut Vec<i32>,used: &mut Vec<bool>, res: &mut Vec<Vec<i32>>){
        if perms.len() == nums.len(){
            res.push(perms.clone());
            return;
        }

        for i in 0..nums.len(){
            if !used[i] {
                used[i] = true;
                perms.push(nums[i]);
                Self::helper(nums, perms, used, res);
                used[i as usize] = false;
                perms.pop();
            }
        }
    }
}