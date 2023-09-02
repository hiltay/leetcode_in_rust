struct Solution;

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut ans = 0;
        for i in 0..n {
            ans ^= start + 2 * i;
        }
        ans
    }
}
fn main() {
    let result = Solution::xor_operation(4, 3);
    println!("{:?}", result);
}
