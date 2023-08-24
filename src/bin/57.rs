struct Solution;
use std::cmp;
impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut left = 0;
        let mut right = 0;
        for i in 0..intervals.len() {
            if new_interval[0] > intervals[i][1] {
                left = i + 1;
            }
            if new_interval[1] >= intervals[i][0] {
                right = i;
            }
        }
        if intervals.len() == left {
            intervals.push(new_interval);
            return intervals;
        } else if intervals[left][0] > new_interval[1] {
            intervals.insert(left, new_interval);
            return intervals;
        }

        intervals[left][0] = cmp::min(intervals[left][0], new_interval[0]);
        intervals[left][1] = cmp::max(intervals[right][1], new_interval[1]);
        for i in left + 1..=right {
            intervals.remove(left + 1);
        }
        intervals
    }
}

fn main() {
    let intervals = vec![
        vec![1, 2],
        vec![3, 5],
        // vec![6, 7],
        // vec![8, 10],
        // vec![12, 16],
    ];
    let new_intervals = vec![0, 0];
    let result = Solution::insert(intervals, new_intervals);
    println!("{:?}", result);
}
