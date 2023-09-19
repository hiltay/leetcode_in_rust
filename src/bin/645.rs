struct Solution;
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut seen = vec![0; n];
        let mut ans = vec![0; 2];
        for i in nums {
            seen[(i - 1) as usize] += 1;
        }
        // 注意这里要返回重复出现的整数，再找到丢失的整数
        // 顺序不能错
        for i in 0..seen.len() {
            if seen[i] == 0 {
                ans[1] = i as i32 + 1;
            } else if seen[i] > 1 {
                ans[0] = i as i32 + 1;
            }
        }

        ans
    }
}

fn main() {
    let nums = vec![2, 2];
    let result = Solution::find_error_nums(nums);
    println!("{:?}", result);
}
