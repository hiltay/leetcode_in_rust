struct Solution;

impl Solution {
    pub fn print_numbers(n: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in 1..=10_i32.pow(n as u32)-1{
            ans.push(i);
        }
        ans
    }
}
fn main() {
    let s = vec![3,2,1];
    let result = Solution::print_numbers(3);
    println!("{:?}", result);
}
