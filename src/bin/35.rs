struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = (left + right) >> 1;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] < target {
                left = mid + 1;
            }else{
                right = mid;
            }

        }
        right as i32


    }
}

fn main() {
    let mut nums = vec![1, 3, 5, 6];
    let result = Solution::search_insert(nums, 5);
    println!("{:?}", result);
}
