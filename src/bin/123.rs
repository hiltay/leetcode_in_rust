use std::cmp;
struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; prices.len()]; 2];
        dp[0][0] = -prices[0];
        dp[1][0] = -prices[0];
        let mut gain1 = 0;
        let mut gain2 = 0;
        for i in 1..prices.len() {
            dp[0][i] = cmp::max(dp[0][i - 1], -prices[i]);
            gain1 = cmp::max(prices[i] + dp[0][i - 1], gain1);
            dp[1][i] = cmp::max(dp[1][i - 1], gain1 - prices[i]);
            gain2 = cmp::max(prices[i] + dp[1][i - 1], gain2);
        }
        cmp::max(gain1,gain2)
    }
}

fn main() {
    let prices = vec![1,2,3,4,5];
    let result = Solution::max_profit(prices);
    println!("{:?}", result);
}
