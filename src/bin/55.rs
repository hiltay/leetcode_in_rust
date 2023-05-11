struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        let mut max_range = nums[0];
        let mut cur = 1;
        while cur <= max_range && cur < nums.len() as i32 {
            if nums[cur as usize] + cur > max_range {
                max_range = nums[cur as usize] + cur;
            }
            cur += 1;
            if cur == nums.len() as i32 {
                return true;
            }
        }
        false
    }
}
fn main() {
    let mut nums = vec![0];
    let result = Solution::can_jump(nums);
    println!("{}", result);
}
