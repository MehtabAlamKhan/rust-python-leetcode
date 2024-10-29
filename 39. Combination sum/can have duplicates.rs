impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combs = Vec::new();
        let mut res = Vec::new();
        Self::helper(target, 0, &candidates, &mut combs, &mut res);
        return res
    }
    pub fn helper(amount: i32, idx: usize, candidates: &Vec<i32>, combs: &mut Vec<i32>, res: &mut Vec<Vec<i32>>){
        if amount == 0{
            res.push(combs.clone());
            return;
        }
        if amount < 0{
            return;
        }

        for i in idx..candidates.len(){
            let c = candidates[i];
            combs.push(c);
            Self::helper(amount - c, i, candidates, combs, res);
            combs.pop();
        }
    }
}