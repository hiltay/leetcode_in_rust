struct Solution;

use std::cmp;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut m = vec![amount + 1; amount as usize + 1];
        m[0] = 0;
        for i in 1..=amount {
            for j in 0..coins.len() {
                if coins[j] <= i {
                    m[i as usize] = cmp::min(m[i as usize], m[(i - coins[j]) as usize] + 1);
                }
            }
        }
        if m[amount as usize] > amount {
            -1
        } else {
            m[amount as usize]
        }
    }
}
fn main() {
    let coins = vec![1, 2, 5];
    let result = Solution::coin_change(coins, 11);
    println!("{}", result);
}
