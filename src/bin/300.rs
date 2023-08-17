struct Solution;
use std::cmp;
// O(n^2)
// impl Solution {
//     pub fn length_of_lis(nums: Vec<i32>) -> i32 {
//         let mut res = 1;
//         let mut dp = vec![1;nums.len()];
//         for i in 1..nums.len() {
//             for j in 0..i{
//                 if nums[j] < nums[i] {
//                     dp[i] = cmp::max(dp[j] + 1, dp[i]);
//                     res = cmp::max(res, dp[i]);
//                 }
//             }
//         }
//         res
//     }
// }

// O(nlogn)
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        let mut arr = Vec::new();
        // init arr
        arr.push(nums[0]);
        for i in 1..nums.len() {
            if nums[i] > *arr.last().unwrap(){
                arr.push(nums[i]);
            }else{
                let mut left = 0;
                let mut right = arr.len()-1;
                while left < right {
                    let mid = (left + right) >> 1;
                    if arr[mid] < nums[i] {
                        left = mid + 1;
                    }else{
                        right = mid;
                    }
                }
                // update arr
                arr[left] = nums[i];
            }
        }
        arr.len() as i32
    }
}

fn main() {
    let nums = vec![10,9,2,5,3,7,101,18];
    let result = Solution::length_of_lis(nums);
    println!("{:?}", result);
}