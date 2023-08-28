struct Solution;
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let sum = n * (1 + n) >> 1;
        sum - nums.iter().sum::<i32>()
    }
}
fn main() {
    let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
    let result = Solution::missing_number(nums);
    println!("{:?}", result);
}
