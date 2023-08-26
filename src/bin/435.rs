struct Solution;
use std::cmp;
// impl Solution {
//     // dp O(n^2)
//     pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
//         // sort the intervals
//         intervals.sort_by_key(|x| x[0]);
//         // 需要保留的最多区间的数量
//         let mut dp = vec![1; intervals.len()];
//         for i in 1..intervals.len() {
//             for j in 0..i{
//                 if intervals[i][0] >= intervals[j][1] {
//                     dp[i] = cmp::max(dp[i], dp[j]+1);
//                 }
//             }
//         }
//         // 需要移除区间的最小数量 == intervals.len() - 需要保留的最多区间的数量
//         (intervals.len() - dp.iter().max().unwrap()) as i32
//     }
// }
impl Solution {
    // 贪心 O(n)
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        // sort with intervals[i][1]
        intervals.sort_by(|x, y| {
            if x[1] != y[1] {
                x[1].cmp(&y[1])
            } else {
                x[0].cmp(&y[0])
            }
        });
        // println!("{:?}", intervals);
        // 可以保留的最多区间的数量
        let mut ans = 1;
        // 之前选择的区间的索引
        let mut before = 0;
        for i in 1..intervals.len() {
            if intervals[i][0] >= intervals[before][1] {
                ans += 1;
                before = i;
            }
        }
        (intervals.len() - ans) as i32
    }
}
fn main() {
    let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
    let result = Solution::erase_overlap_intervals(intervals);
    println!("{:?}", result);
}
