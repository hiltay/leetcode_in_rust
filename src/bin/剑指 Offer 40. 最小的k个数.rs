struct Solution;
use std::convert::TryInto;
impl Solution {
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut sorted = arr.clone();
        sorted.sort();
        sorted[0..k as usize].try_into().unwrap()
    }
}
fn main() {
    let s = vec![3,2,1];
    let result = Solution::get_least_numbers(s,2);
    println!("{:?}", result);
}
