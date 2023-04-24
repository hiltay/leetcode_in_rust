struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut left = 0;
        let mut right = if nums.len() == 0 {
            return 0;
        } else {
            nums.len() - 1
        };
        while left != right {
            if nums[left] == val {
                nums[left] = nums[right];
                right -= 1;
            } else {
                left += 1;
            }
        }
        if nums[left] == val {
            left as i32
        } else {
            left as i32 + 1
        }
    }
}
fn main() {
    // nums = [0,1,2,2,3,0,4,2], val = 2
    let mut nums = vec![1];
    let result = Solution::remove_element(&mut nums, 1);
    println!("{}", result);
    for i in nums.iter() {
        println!("{}", i);
    }
}
