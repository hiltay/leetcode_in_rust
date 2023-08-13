use std::cmp;
struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0;prices.len()];2];
        dp[0][0] = -prices[0];
        dp[1][0] = 0;
        let mut gain = 0;
        for i in 1..prices.len() {
            dp[0][i] = cmp::max(dp[0][i-1], if i > 1{dp[1][i-2]-prices[i]}else{-prices[i]});
            dp[1][i] = cmp::max(prices[i]+dp[0][i-1],dp[1][i-1]);
            gain = cmp::max(dp[1][i],gain)
        }
        gain
    }
}

fn main() {
    let prices = vec![2,1,4];
    let result = Solution::max_profit(prices);
    println!("{:?}", result);
}
