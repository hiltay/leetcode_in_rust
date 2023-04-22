struct Solution;
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        return n > 0 && (n & -n) == n;
    }
}
fn main() {
    // 1 == 0b00000001
    // 3 == 0b00000011
    // 4 == 0b00000100
    let result = Solution::is_power_of_two(5);
    println!("{}", result);
}
