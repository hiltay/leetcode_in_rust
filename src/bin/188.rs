use std::cmp;
struct Solution;
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; prices.len()]; 2 * k as usize];
        let mut gain = 0;
        for i in 0..prices.len() {
            for j in 0..2 * k as usize {
                if j & 1 > 0 {
                    if i == 0 {
                        continue;
                    }
                    dp[j][i] = cmp::max(prices[i] + dp[j - 1][i - 1], dp[j][i - 1]);
                    gain = cmp::max(gain, dp[j][i]);
                } else {
                    dp[j][i] = if i == 0 {
                        -prices[i]
                    } else if j == 0 {
                        cmp::max(-prices[i], dp[j][i - 1])
                    } else {
                        cmp::max(dp[j - 1][i - 1] - prices[i], dp[j][i - 1])
                    };
                }
            }
        }
        gain
    }
}

fn main() {
    let prices = vec![3, 2, 6, 5, 0, 3];
    let result = Solution::max_profit(1, prices);
    println!("{:?}", result);
}
