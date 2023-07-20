use std::cmp::{self, min};
struct Solution;

impl Solution {
    fn recursive(grid: &Vec<Vec<i32>>, k: usize, j: usize, mut mem: &mut Vec<Vec<i32>>) -> i32 {
        if mem[k][j] != -1 {
            return mem[k][j];
        } else {
            let min_dis;
            if k < grid.len() - 1 && j < grid[0].len() - 1 {
                min_dis = cmp::min(
                    Solution::recursive(&grid, k + 1, j, &mut mem),
                    Solution::recursive(&grid, k, j + 1, &mut mem),
                );
            } else if k < grid.len() - 1 {
                min_dis = Solution::recursive(&grid, k + 1, j, &mut mem);
            } else if j < grid[0].len() - 1 {
                min_dis = Solution::recursive(&grid, k, j + 1, &mut mem);
            }else{
                return grid[k][j];
            }
            mem[k][j] = min_dis + grid[k][j];
            return mem[k][j];
        }
    }

    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut mem = vec![vec![]; grid.len()];
        for i in 0..grid.len() {
            mem[i] = vec![-1; grid[i].len()];
        }
        return Solution::recursive(&grid, 0, 0, &mut mem);
    }
}

fn main() {
    let nums = vec![vec![1, 2, 3], vec![4,5,6], ];
    let result = Solution::min_path_sum(nums);
    println!("{}", result);
}
