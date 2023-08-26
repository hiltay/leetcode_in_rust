struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = (left + right) >>1;
            if nums[mid] == mid as i32{
                left = mid +1;
            }else{
                right = mid;
            }
        }
        right as i32
    }
}
fn main() {
    let s = vec![0,1,2,3,4,5,6,7,9];
    let result = Solution::missing_number(s);
    println!("{:?}", result);
}
