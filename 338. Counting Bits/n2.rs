impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        for i in 0..n + 1 {
            res.push(Self::helper(i as i32));
        }
        res
    }
    pub fn helper(n: i32) -> i32 {
        let mut n = n;
        let mut res = 0;
        while n > 0 {
            res += n % 2;
            n = n >> 1;
        }
        res
    }
}
