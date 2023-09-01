struct Solution;

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let entropy = (minutes_to_test / minutes_to_die) + 1;
        ((buckets as f32).ln() / (entropy as f32).ln()).ceil() as i32
    }
}
fn main() {
    let buckets = 1000;
    let minutesToDie = 15;
    let minutesToTest = 60;
    let result = Solution::poor_pigs(buckets, minutesToDie, minutesToTest);
    println!("{:?}", result);
}
