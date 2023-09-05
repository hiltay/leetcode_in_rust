struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut min1 = 10;
        let mut min2 = 10;
        let mut set = HashSet::new();
        let mut duplicates_num = 10;
        for i in nums1 {
            if min1 > i {
                min1 = i;
            }
            set.insert(i);
        }

        for i in nums2 {
            if set.contains(&i) && duplicates_num > i {
                duplicates_num = i;
            }
            if min2 > i {
                min2 = i;
            }
        }
        if duplicates_num != 10 {
            duplicates_num
        } else if min1 < min2 {
            min2 + min1 * 10
        } else {
            min1 + min2 * 10
        }
    }
}
fn main() {
    let nums1 = vec![4, 3, 2, 6];
    let nums2 = vec![4, 3, 7];
    let result = Solution::min_number(nums1, nums2);
    println!("{:?}", result);
}
