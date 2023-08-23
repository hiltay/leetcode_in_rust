struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        let mut duplicates = HashSet::new();
        for i in nums{
            if !set.contains(&i) && !duplicates.contains(&i){
                set.insert(i);
            }else{
                set.remove(&i);
                duplicates.insert(i);
            }
        }
        set.iter().sum::<i32>()
    }
}

fn main() {
    let nums = vec![1,2,3,2,2];
    let result = Solution::sum_of_unique(nums);
    println!("{}", result);
}