struct Solution;
impl Solution {
    fn check_next(
        row: usize,
        col: usize,
        grid: &Vec<Vec<char>>,
        mut visited: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if row >= grid.len() {
            return 0;
        }
        if col >= grid[0].len() {
            return 0;
        }
        if visited[row][col] == 1 {
            return 0;
        }
        visited[row][col] = 1;
        let cur_type = grid[row][col];

        if cur_type == '1' {
            // island
            Solution::check_next(row + 1, col, &grid, &mut visited);
            if row > 0 {
                Solution::check_next(row - 1, col, &grid, &mut visited);
            }
            if col > 0 {
                Solution::check_next(row, col - 1, &grid, &mut visited);
            }
            Solution::check_next(row, col + 1, &grid, &mut visited);

            return 1;
        } else {
            return 0;
        }
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut visited = vec![vec![0; grid[0].len()]; grid.len()];
        let mut res = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                res += Solution::check_next(i, j, &grid, &mut visited);
            }
        }

        res
    }
}

fn main() {
    let grid = vec![
        vec!['1', '0', '1', ],
        vec!['0', '0', '1', ],
        vec!['1', '0', '1', ],
    ];
    let result = Solution::num_islands(grid);
    println!("{}", result);
}
