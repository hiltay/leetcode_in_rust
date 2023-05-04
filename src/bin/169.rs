struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cur_bit_count = 0;
        let mut result = 0;
        for cur_bit in 0..31 {
            let mask = 2_i32.pow(cur_bit);
            for n in nums.iter() {
                if n & mask != 0 {
                    cur_bit_count += 1;
                }
            }
            if cur_bit_count > nums.len() / 2 {
                result += mask;
            }
            cur_bit_count = 0;
        }
        result
    }
}
fn main() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2,1,1,1,1];
    let result = Solution::majority_element(nums);
    println!("{}", result);
}
