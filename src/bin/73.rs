struct Solution;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let length = matrix.len();
        let width = matrix[0].len();
        let mut col = vec![0; length];
        let mut row = vec![0; width];
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    col[i] = 1;
                    row[j] = 1;
                }
            }
        }
        println!("{:?}", col);
        println!("{:?}", row);
        for i in 0..col.len() {
            if col[i] == 1 {
                for j in 0..width {
                    matrix[i][j] = 0;
                }
            }
        }
        for i in 0..row.len() {
            if row[i] == 1 {
                for j in 0..length {
                    matrix[j][i] = 0;
                }
            }
        }
    }
}
fn main() {
    // let mut matrix = vec![vec![1, 2, 0, 4], vec![1, 1, 5, 1], vec![3, 2, 6, 0]];
    let mut matrix = vec![vec![0,1,2,0], vec![3,4,5,2], vec![1,3,1,5]];
    Solution::set_zeroes(&mut matrix);
    println!("{:?}", matrix);
}
