struct Solution;
use std::cmp;
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        intervals.sort_by_key(|x| x[0]);
        ans.push(intervals[0].clone());
        for i in 1..intervals.len() {
            if ans.last().unwrap()[1] >= intervals[i][0] {
                ans.last_mut().unwrap()[1] = cmp::max(ans.last().unwrap()[1], intervals[i][1])
            } else {
                ans.push(intervals[i].clone());
            }
        }
        ans
    }
}

fn main() {
    let intervals = vec![vec![1, 4], vec![4, 5]];
    let result = Solution::merge(intervals);
    println!("{:?}", result);
}
