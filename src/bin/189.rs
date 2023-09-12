struct Solution;
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let length = nums.len();
        // i -> (i+k) % length
        let mut ans = vec![0; length];
        for i in 0..length {
            ans[(i + k as usize) % length] = nums[i];
        }
        for i in 0..length {
            nums[i] = ans[i]
        }
    }
}
fn main() {
    let mut nums = vec![-1, -100, 3, 99];
    Solution::rotate(&mut nums, 10);
    println!("{:?}", nums);
}
