impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        let mut carry = 0;
        while b != 0 {
            carry = (a & b) << 1;
            a = a ^ b;
            b = carry;
        }
        a
    }
}
