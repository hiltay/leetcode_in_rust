use std::vec;

struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![-1; obstacle_grid[0].len()]; obstacle_grid.len()];
        for i in 0..obstacle_grid.len() {
            for j in 0..obstacle_grid[i].len() {
                if i == 0 && j == 0 {
                    if obstacle_grid[i][j] != 1 {
                        dp[i][j] = 1;
                    } else {
                        dp[i][j] = 0;
                    }
                } else if i == 0 {
                    if obstacle_grid[i][j] != 1 {
                        dp[i][j] = dp[i][j - 1];
                    } else {
                        dp[i][j] = 0;
                    }
                } else if j == 0 {
                    if obstacle_grid[i][j] != 1 {
                        dp[i][j] = dp[i - 1][j];
                    } else {
                        dp[i][j] = 0;
                    }
                } else {
                    if obstacle_grid[i][j] != 1 {
                        dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                    } else {
                        dp[i][j] = 0;
                    }
                }
            }
        }
        return dp[obstacle_grid.len() - 1][obstacle_grid[0].len() - 1];
    }
}

fn main() {
    let obstacle_grid: Vec<Vec<i32>> = vec![vec![1, 0]];
    let result = Solution::unique_paths_with_obstacles(obstacle_grid);
    println!("{:?}", result);
}
