struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }else if x == 1{
            return 1;
        }
        let target = x as i64;
        let mut left:i64 = 1;
        let mut right:i64 = x as i64;
        let mut mid:i64;
        // let mut last_t = 0; // left > left > left
        while left < right-1{
            mid = (left + right) >> 1;
            let pow = mid * mid;
            if pow == target {
                return mid as i32;
            } else if pow < target {
                left = mid;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}

fn main() {
    let result = Solution::my_sqrt(1);
    println!("{}", result);
}
