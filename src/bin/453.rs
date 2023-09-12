struct Solution;
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = *nums.iter().min().unwrap();
        let mut ans = 0;
        for i in nums {
            ans += i - min;
        }
        ans
    }
}

fn main() {
    let nums = vec![1, 1, 1000000000];
    let result = Solution::min_moves(nums);
    println!("{:?}", result);
}
