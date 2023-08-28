struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in nums {
            if map.contains_key(&(i % space)) {
                map.get_mut(&(i % space)).unwrap().push(i);
            } else {
                map.insert(i % space, vec![i]);
            }
        }
        let mut ans = 0;
        let mut length = 0;
        for i in map.values() {
            let min_ = *i.iter().min().unwrap();
            let len = i.len();
            // 目标数量最多的，如果目标数相等，选择最小的
            if len > length || len == length && ans > min_ {
                length = len;
                ans = min_;
            }
        }
        ans
    }
}

fn main() {
    let nums = vec![1, 3, 5, 2, 4, 6];
    let result = Solution::destroy_targets(nums, 2);
    println!("{}", result);
}
