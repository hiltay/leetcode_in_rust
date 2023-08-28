struct Solution;
impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        let mut ans = Vec::new();
        ans.push(celsius+273.15);
        ans.push(celsius*1.8 + 32.00);
        ans
    }
}
fn main() {
    let result = Solution::convert_temperature(36.5);
    println!("{:?}", result);
}
