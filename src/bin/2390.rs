struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack = Vec::new();
        for i in s.chars() {
            if i != '*' {
                stack.push(i);
            } else {
                stack.pop();
            }
        }
        stack.iter().collect()
    }
}
fn main() {
    let s = "erase*****".to_string();
    let result = Solution::remove_stars(s);
    println!("{:?}", result);
}
