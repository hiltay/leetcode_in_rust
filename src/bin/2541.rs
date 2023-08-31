struct Solution;

impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        if nums1.iter().sum::<i32>() != nums2.iter().sum::<i32>() {
            return -1;
        }
        // 一定要注意这里用i64，如果用i32会发生溢出，倒数第二个测试用例无法通过！
        let k = k as i64;
        let mut left:i64 = 0;
        let mut right:i64 = 0;
        for i in 0..nums1.len() {
            let sub = (nums2[i] - nums1[i]) as i64;
            if k != 0{
                if sub < 0 && sub % k == 0 {
                    left += sub / k;
                } else if sub > 0 && sub % k == 0 {
                    right += sub / k;
                }else if sub & k != 0{
                    return -1;
                }
            }else if sub != 0{
                return -1;
            }
            
        }
        if left.abs() == right {
            right
        } else {
            -1
        }
    }
}
fn main() {
    let nums1 = vec![10,5,15,20];
    let nums2 = vec![20,10,15,5];
    let result = Solution::min_operations(nums1, nums2, 0);
    println!("{:?}", result);
}
