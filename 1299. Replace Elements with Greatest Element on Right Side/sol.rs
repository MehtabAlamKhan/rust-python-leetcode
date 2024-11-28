impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let ln = arr.len();
        let mut arr = arr;
        let mut mx = -1;
        for i in (0..arr.len()).rev() {
            let tmp = arr[i];
            arr[i] = mx;
            mx = mx.max(tmp);
        }
        arr
    }
}
