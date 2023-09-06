struct Solution;
// wrong answer!!
// impl Solution {
//     pub fn predict_the_winner(nums: Vec<i32>) -> bool {
//         let mut score1 = 0;
//         let mut score2 = 0;
//         let mut left = 0;
//         let mut right = nums.len();
//         let mut turns = 1;

//         while left < right {
//             let slice = &nums[left..right];
//             println!("{:?}", slice);
//             let cur_max_pos = nums
//                 .iter()
//                 .position(|&x| x == *slice.iter().max().unwrap())
//                 .unwrap();

//             if turns & 1 == 1 {
//                 // player 1 turns
//                 if left == cur_max_pos {
//                     score1 += nums[left];
//                     left += 1;
//                 } else if right - 1 == cur_max_pos {
//                     score1 += nums[right - 1];
//                     right -= 1;
//                 } else {
//                     if left + 1 < cur_max_pos && right - 2 > cur_max_pos {
//                         // 选择当前最大的
//                         if nums[left] > nums[right - 1] {
//                             score1 += nums[left];
//                             left += 1;
//                         } else {
//                             score1 += nums[right - 1];
//                             right -= 1;
//                         }
//                     } else if right - 2 > cur_max_pos {
//                         // 只有选右边不会暴露最大元素
//                         score1 += nums[right - 1];
//                         right -= 1;
//                     } else if left + 1 < cur_max_pos {
//                         // 只有选左边不会暴露最大元素
//                         score1 += nums[left];
//                         left += 1;
//                     } else {
//                         // 也有可能无法获取到最大的那个元素，选择当前最大的
//                         if nums[left] > nums[right - 1] {
//                             score1 += nums[left];
//                             left += 1;
//                         } else {
//                             score1 += nums[right - 1];
//                             right -= 1;
//                         }
//                     }
//                 }
//             } else {
//                 // player 2 turns
//                 if left == cur_max_pos {
//                     score2 += nums[left];
//                     left += 1;
//                 } else if right - 1 == cur_max_pos {
//                     score2 += nums[right - 1];
//                     right -= 1;
//                 } else {
//                     if left + 1 < cur_max_pos && right - 2 > cur_max_pos {
//                         // 选择当前最大的
//                         if nums[left] > nums[right - 1] {
//                             score2 += nums[left];
//                             left += 1;
//                         } else {
//                             score2 += nums[right - 1];
//                             right -= 1;
//                         }
//                     } else if right - 2 > cur_max_pos {
//                         // 只有选右边不会暴露最大元素
//                         score2 += nums[right - 1];
//                         right -= 1;
//                     } else if left + 1 < cur_max_pos {
//                         // 只有选左边不会暴露最大元素
//                         score2 += nums[left];
//                         left += 1;
//                     } else {
//                         // 也有可能无法获取到最大的那个元素，选择当前最大的
//                         if nums[left] > nums[right - 1] {
//                             score2 += nums[left];
//                             left += 1;
//                         } else {
//                             score2 += nums[right - 1];
//                             right -= 1;
//                         }
//                     }
//                 }
//             }
//             turns += 1;
//         }
//         score1 >= score2
//     }
// }

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {}
}
fn main() {
    let nums = vec![
        1921045, 6, 5132440, 5, 3, 6610604, 7, 8650002, 6337645, 3740419, 5242495, 3729694, 1,
        4293537, 3, 2, 5, 9278, 4,
    ];
    let result = Solution::predict_the_winner(nums);
    println!("{:?}", result);
}
