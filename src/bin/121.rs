use std::cmp;
struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut dp = vec![0;prices.len()];
        dp[0] = -prices[0];
        for i in 1..prices.len() {
            dp[i] = cmp::max(dp[i-1], -prices[i]);
            result = cmp::max(result, prices[i] + dp[i-1])
        }
        result
    }
}

fn main() {
    let prices = vec![7];
    let result = Solution::max_profit(prices);
    println!("{:?}", result);
}
