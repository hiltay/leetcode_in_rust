struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut appear_bit_num = 0;
        let mut mask = 1;
        let mut result = 0;
        for _ in 0..=31 {
            for item in nums.iter() {
                if item & mask != 0 {
                    appear_bit_num += 1;
                }
            }
            if appear_bit_num % 3 == 1 {
                result += mask;
            }
            appear_bit_num = 0;
            mask <<= 1;
        }
        result
    }
}
fn main() {
    // 1010
    // 1010
    // 1010
    // 0011
    // 1001
    // 1001
    // 1001
    // 1111
    // 1111
    // 1111
    let nums = vec![0,1,0,1,0,1,99];
    let result = Solution::single_number(nums);
    println!("{:?}", result);
}
