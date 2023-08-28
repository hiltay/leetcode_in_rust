struct Solution;
impl Solution {
    pub fn min_impossible_or(nums: Vec<i32>) -> i32 {
        let mut mask = 0;
        for i in nums {
            if i & (i - 1) == 0 {
                mask |= i;
            }
        }
        mask = !mask;
        mask & -mask
    }
}
fn main() {
    let nums = vec![5, 3, 2];
    let result = Solution::min_impossible_or(nums);
    println!("{:?}", result);
}
