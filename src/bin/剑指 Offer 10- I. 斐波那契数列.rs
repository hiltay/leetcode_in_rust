struct Solution;
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        } else {
            let mut previous: i128 = 0;
            let mut current = 1;
            for _ in 0..n - 1 {
                let new_fib = previous + current;
                previous = current;
                current = new_fib;
            }
            return (current % 1000000007) as i32;
        }
    }
}
fn main() {
    let s: Vec<i32> = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let result = Solution::fib(s, 3);
    println!("{:?}", result);
}
