struct Solution;
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut ans = 0;
        for i in 1..time_series.len() {
            if time_series[i - 1] + duration <= time_series[i] {
                ans += duration;
            } else {
                ans += time_series[i] - time_series[i - 1]
            }
        }
        ans + duration
    }
}
fn main() {
    let time_series = vec![1, 2];
    let result = Solution::find_poisoned_duration(time_series, 2);
    println!("{:?}", result);
}
