use std::collections::HashMap;
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        return Self::sum(0, 0, &mut HashMap::new(), &coins, &amount)
    }
    pub fn sum(i: i32, total: i32, memo: &mut HashMap<(i32, i32), i32>, coins: &Vec<i32>, amount: &i32) -> i32{
        if let Some(&cached) = memo.get(&(i, total)){
            return cached;
        }
        if i as usize >= coins.len() || total > *amount{
            return 0;
        }
        if total == *amount{
            return 1;
        }
        let res = Self::sum(i, total + coins[i as usize], memo, coins, amount) + Self::sum(i + 1, total, memo, coins, amount);
        memo.insert((i, total), res);
        return res;
    }
}