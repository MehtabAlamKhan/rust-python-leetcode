use std::collections::HashMap;
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();
        let mut nums = nums.clone();
        nums.insert(0, 1);
        nums.push(1);
        Self::helper(1, nums.len() - 2, &nums, &mut memo)
    }
    pub fn helper(
        l: usize,
        r: usize,
        nums: &Vec<i32>,
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if let Some(&cached) = memo.get(&(l, r)) {
            return cached;
        }
        if l > r {
            return 0;
        }
        let mut maxCoins = 0;
        for i in l..r + 1 {
            let mut coins = nums[l - 1] * nums[i] * nums[r + 1];
            coins += Self::helper(l, i - 1, nums, memo) + Self::helper(i + 1, r, nums, memo);
            maxCoins = maxCoins.max(coins);
        }
        memo.insert((l, r), maxCoins);
        maxCoins
    }
}
