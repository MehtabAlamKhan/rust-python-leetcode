impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut cans: Vec<i32> = Vec::new();
        let mut nums = nums;
        let mut visited: Vec<bool> = vec![false; nums.len()];
        nums.sort_unstable();
        Solution::dfs_helper(nums, &mut cans, &mut res, &mut visited);
        res
    }

    fn dfs_helper(
        nums: Vec<i32>,
        cans: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        visited: &mut Vec<bool>,
    ) {
        if cans.len() == nums.len() {
            res.push(cans.clone());
            return;
        }

        for (idx, &num) in nums.iter().enumerate() {
            // check dup entry
            if visited[idx] || idx > 0 && nums[idx] == nums[idx - 1] && !visited[idx - 1] {
                continue;
            }

            visited[idx] = true;
            cans.push(num);
            Solution::dfs_helper(nums.clone(), cans, res, visited);
            cans.pop();
            visited[idx] = false;
        }
    }
}
