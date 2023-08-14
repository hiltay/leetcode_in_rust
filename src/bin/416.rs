struct Solution;
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return false;
        };
        let sum: i32 = nums.iter().sum();
        if sum & 1 == 0 {
            // even number
            let mut dp = vec![false; (sum >> 1) as usize + 1];
            for i in 0..=(sum >> 1) as usize {
                dp[i] = nums[0] == i as i32;
            }
            for i in 1..nums.len() {
                for j in (0..=(sum >> 1) as usize).rev() {
                    if j as i32 >= nums[i] {
                        dp[j] = dp[j] || (dp[j - nums[i] as usize]);
                    }
                }
            }
            return dp[(sum >> 1) as usize];
        }
        false
    }
}

fn main() {
    let nums = vec![1, 5, 11, 5];
    let result = Solution::can_partition(nums);
    println!("{}", result);
}
