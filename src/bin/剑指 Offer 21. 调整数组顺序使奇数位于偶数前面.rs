struct Solution;
impl Solution {
    pub fn exchange(mut nums: Vec<i32>) -> Vec<i32> {
        let mut left = 0;
        let mut right = nums.len()-1;
        if nums.len() == 0{
            return nums;
        }
        while left < right {
            while left < right && nums[left] & 1==1 {
                left +=1
            }
            while left < right && nums[right] & 1 ==0 {
                right -=1;
            }
            nums.swap(left, right);
            
        }
        nums
    }
}
fn main() {
    let s: Vec<i32> = vec![1,2,3,4];
    let result = Solution::exchange(s);
    println!("{:?}", result);
}
