struct Solution;
impl Solution {
    pub fn minimum_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut ans = 0;
        let mut prev = -1;
        for i in nums {
            if i != 0 {
                if prev != i {
                    prev = i;
                    ans += 1;
                }
            }
        }
        ans
    }
}
fn main() {
    let nums = vec![0];
    let result = Solution::minimum_operations(nums);
    println!("{:?}", result);
}
