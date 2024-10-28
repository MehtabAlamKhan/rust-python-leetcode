use std::collections::HashMap;
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        return Self::ways(0, 0, &nums, &target, &mut HashMap::new())
    }
    pub fn ways(i: i32, total: i32, nums: &Vec<i32>, target: &i32, memo: &mut HashMap<(i32, i32), i32>)->i32{
        if let Some(&cached) = memo.get(&(i, total)){
            return cached;
        }
        if i as usize == nums.len() && total == *target{
            return 1;
        }
        if i as usize == nums.len() && total != *target{
            return 0;
        }
        let res = Self::ways(i + 1, total + nums[i as usize], nums, target, memo) + 
                    Self::ways(i + 1, total - nums[i as usize], nums, target, memo);
        memo.insert((i, total), res);
        return res;
    }
}