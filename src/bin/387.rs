struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut order = 0;
        let s_arr: Vec<char> = s.chars().collect();
        for i in 0..s_arr.len() {
            if map.contains_key(&s_arr[i]) {
                map.insert(s_arr[i], -1);
            } else {
                map.insert(s_arr[i], order);
                order += 1;
            }
        }

        let mut ans = -1;
        let mut ans_v = i32::MAX;
        for (k, v) in map {
            if ans_v > v && v != -1 {
                ans_v = v;
                ans = s_arr.iter().position(|&x| x==k).unwrap() as i32;
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
