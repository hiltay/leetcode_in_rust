struct Solution;
// Using bit manipulation approach
// impl Solution {
//     pub fn majority_element(nums: Vec<i32>) -> i32 {
//         let mut cur_bit_count = 0;
//         let mut result = 0;
//         let mut mask = 0;
//         let half_length = nums.len() / 2;
//         for _ in 0..31 {
//             mask = if mask != 0 { mask << 1 } else { 1 };
//             for n in nums.iter() {
//                 if n & mask != 0 {
//                     cur_bit_count += 1;
//                 }
//             }
//             if cur_bit_count > half_length {
//                 result += mask;
//             }
//             cur_bit_count = 0;
//         }
//         result
//     }
// }

// Efficient approach using Boyer-Moore Voting algorithm
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut majority_candidate = 0;
        let mut count = 0;
        for item in nums {
            if count == 0 {
                majority_candidate = item;
            }

            if item == majority_candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }
        majority_candidate
    }
}
fn main() {
    let nums = vec![3, 3, 4];
    let result = Solution::majority_element(nums);
    println!("{}", result);
}
