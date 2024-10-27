use std::collections::HashMap;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        return Self::sell(0, true, &prices, &mut HashMap::new())
    }
    pub fn sell(i: i32, buying: bool, prices: &Vec<i32>, memo: &mut HashMap<(i32, bool), i32>) -> i32{
        if i as usize >= prices.len(){
            return 0;
        }
        if let Some(&cached) = memo.get(&(i, buying)){
            return cached;
        }
        let mut res = 0;
        let cooldown = Self::sell(i + 1, buying, prices, memo);
        if buying{
            res = cooldown.max(Self::sell(i + 1, !buying, prices, memo) - prices[i as usize]);
        }
        else{
            res = cooldown.max(Self::sell(i + 2, !buying, prices, memo) + prices[i as usize]);
        }
        memo.insert((i, buying), res);
        return res;
    }
}