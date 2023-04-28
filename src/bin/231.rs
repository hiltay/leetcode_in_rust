use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut set:HashSet<i32> = HashSet::new();
        for i in nums{
            if set.contains(&i){
                set.remove(&i);
            }else{
                set.insert(i);
            }
        }
        let res = set.drain().next().unwrap();
        res
        
    }
}
fn main() {
    let nums = vec![4,1,2,1,2];
    let result = Solution::single_number(nums);
    println!("{}", result);
}
