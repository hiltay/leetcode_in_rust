struct Solution;
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut ans = vec![0; temperatures.len()];
        for i in 0..temperatures.len() {
            if stack.is_empty() {
                stack.push(i)
            } else {
                while !stack.is_empty() && temperatures[*stack.last().unwrap()] < temperatures[i] {
                    let idx = stack.pop().unwrap();
                    ans[idx] = (i - idx) as i32;
                }
                stack.push(i);
            }
        }
        ans
    }
}
fn main() {
    let s = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let result = Solution::daily_temperatures(s);
    println!("{:?}", result);
}
