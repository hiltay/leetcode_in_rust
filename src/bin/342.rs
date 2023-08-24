struct Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        if n & (n - 1) == 0 {
            if (n - 1) % 3 == 0 {
                return true;
            }
        }
        false
    }
}

fn main() {
    // 0000 0001
    // 0000 0100
    // 0001 0000
    // 0100 0000
    let result = Solution::is_power_of_four(64);
    println!("{}", result);
}
