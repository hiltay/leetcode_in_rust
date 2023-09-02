struct Solution;
use std::cmp;
impl Solution {
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        let mut pos: i32 = -1;
        let mut ans = 0;
        for i in 0..forts.len() {
            if forts[i] != 0 {
                if pos >= 0 && forts[pos as usize] != forts[i] {
                    ans = cmp::max(i - 1 - pos as usize, ans)
                }
                pos = i as i32;
            }
        }
        ans as i32
    }
}
fn main() {
    let forts = vec![0, 0, 1, -1];
    let result = Solution::capture_forts(forts);
    println!("{:?}", result);
}
