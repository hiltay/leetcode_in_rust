struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut record = HashSet::new();
        for i in nums {
            if record.contains(&i){
                return i;
            }else {
                record.insert(i);
            }
        }
        0
    }
}
fn main() {
    let s: Vec<i32> = vec![2, 3, 1, 0, 2, 5, 3];
    let result = Solution::find_repeat_number(s);
    println!("{:?}", result);
}
