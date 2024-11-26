impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut reversed = 0;

        while x != 0 {
            let digit = x % 10;
            x /= 10;

            if reversed > i32::MAX / 10 || (reversed == i32::MAX / 10 && digit > 7) {
                return 0;
            }
            if reversed < i32::MIN / 10 || (reversed == i32::MIN / 10 && digit < -8) {
                return 0;
            }

            reversed = reversed * 10 + digit;
        }

        reversed
    }
}
