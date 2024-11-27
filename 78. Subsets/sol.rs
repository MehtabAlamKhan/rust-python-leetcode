impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut sub: Vec<i32> = Vec::new();
        Self::helper(0, &mut res, &mut sub, &nums);
        res
    }

    pub fn helper(i: usize, res: &mut Vec<Vec<i32>>, sub: &mut Vec<i32>, nums: &Vec<i32>) {
        if i >= nums.len() {
            res.push(sub.clone());
            return;
        }
        sub.push(nums[i]);
        Self::helper(i + 1, res, sub, nums);
        sub.pop();
        Self::helper(i + 1, res, sub, nums);
    }
}
