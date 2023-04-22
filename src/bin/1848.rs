struct Solution;
impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let mut left;
        let mut right;
        let mut current = start;
        for range in 0..nums.len() as i32 {
            println!("{}", range);
            left = current - range;
            if left >= 0{
                if nums[left as usize] == target{
                    current = left;
                    break;
                }
            }
            right = current + range;
            if right < nums.len() as i32 {
                if nums[right as usize] == target{
                    current = right;
                    break;
                }
            }

        }
        
        (current-start).abs()
    }
}
fn main() {
    let nums = vec![1,1,1,1];
    let result = Solution::get_min_distance(nums, 1, 0);
    println!("{}", result);
}
