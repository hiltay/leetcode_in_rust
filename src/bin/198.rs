use std::cmp;

struct Solution;
// impl Solution {
//     fn recursive(nums: &Vec<i32>, pos: usize, mut mem: &mut Vec<i32>) -> i32 {
//         if pos >= nums.len() {
//             return 0;
//         }

//         if mem[pos] != -1 {
//             return mem[pos];
//         } else {
//             let mut max = 0;
//             for i in pos..nums.len() {
//                 max = cmp::max(nums[i] + Solution::recursive(&nums, i + 2, &mut mem), max);
//             }
//             mem[pos] = max;
//             return max;
//         }
//     }

//     pub fn rob(nums: Vec<i32>) -> i32 {
//         let mut mem = vec![-1; nums.len()];
//         Solution::recursive(&nums, 0, &mut mem)
//     }
// }

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let size = nums.len();
        if size < 2 {
            let mut max = 0;
            for i in nums{
                max = cmp::max(max, i)
            }
            return max;
        }
        let mut mem = vec![-1; size];

        mem[size - 1] = nums[size - 1];
        for i in (0..=size - 2).rev() {
            for j in i..size {
                mem[i] = cmp::max(mem[i], nums[j] + if j + 2 < size { mem[j + 2] } else { 0 })
            }
        }
        mem[0]
    }
}

fn main() {
    let nums = vec![2,1,1,2];
    let result = Solution::rob(nums);
    println!("{:?}", result);
}
