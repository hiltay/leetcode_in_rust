struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> char {
        let mut map = HashMap::new();
        let mut index = 0;
        for i in s.chars(){
            if map.contains_key(&i){
                map.insert(i, -1);
            }else{
                map.insert(i, index);
                index +=1;
            }
        }
        let mut ans= ' ';
        let mut ans_v= i32::MAX;
        for (k,v) in map{
            if ans_v > v && v != -1{
                ans_v = v;
                ans = k;
            }
        }
        ans
    }
}
fn main() {
    let s = "aadadaad".to_string();
    let result = Solution::first_uniq_char(s);
    println!("{:?}", result);
}
