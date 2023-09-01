struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut cur_num = num;
        let mut ans = 0;
        while cur_num != 0 {
            if cur_num & 1 == 0 {
                cur_num >>= 1;
            } else {
                cur_num -= 1;
            }
            ans += 1;
        }
        ans
    }
}
fn main() {
    let result = Solution::number_of_steps(8);
    println!("{:?}", result);
}
