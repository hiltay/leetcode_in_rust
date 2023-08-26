struct Solution;
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; matrix.len()]; matrix[0].len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                ans[j][i] = matrix[i][j];
            }
        }

        ans
    }
}
fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let result = Solution::transpose(matrix);
    println!("{:?}", result);
}
