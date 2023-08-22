struct Solution;
impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        if n <= 6 {
            return true;
        }

        let mut num = n;
        while num % 2 == 0 {
            num >>= 1
        }
        while num % 3 == 0 {
            num /= 3;
        }
        while num % 5 == 0 {
            num /= 5;
        }
        num == 1
    }
}
fn main() {
    let result = Solution::is_ugly(0);
    println!("{}", result);
}
