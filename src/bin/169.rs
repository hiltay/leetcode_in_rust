struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cur_bit_count = 0;
        let mut result = 0;
        let mut mask = 0;
        let half_length = nums.len()/2;
        for _ in 0..31 {
            mask = if mask != 0 {
                mask << 1
            }else{
                1
            };
            for n in nums.iter() {
                if n & mask != 0 {
                    cur_bit_count += 1;
                }
            }
            if cur_bit_count > half_length {
                result += mask;
            }
            cur_bit_count = 0;
        }
        result
    }
}
fn main() {
    let nums = vec![3,2,3];
    let result = Solution::majority_element(nums);
    println!("{}", result);
}
