struct Solution;
impl Solution {
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        let mut cur_num = target;
        let mut ans = 0;
        while cur_num > start_value {
            if cur_num & 1 == 0 {
                cur_num >>= 1;
            } else {
                cur_num += 1;
            }
            ans += 1;
        }
        ans + start_value - cur_num
    }
}

fn main() {
    let result = Solution::broken_calc(1, 1000000000);
    println!("{:?}", result);
}
