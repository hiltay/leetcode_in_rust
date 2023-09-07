struct Solution;
impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut left = 0;
        let mut right = i64::MAX;
        while left < right {
            let mid = ((left + right) >> 1) as f64;
            let mut repairs: i64 = 0;
            for i in ranks.iter() {
                repairs += (mid / *i as f64).sqrt().floor() as i64;
            }
            if repairs >= cars as i64 {
                right = mid as i64;
            } else {
                left = mid as i64 + 1;
            }
        }
        left
    }
}
fn main() {
    let ranks = vec![4, 2, 3, 1];
    let result = Solution::repair_cars(ranks, 10);
    println!("{:?}", result);
}
