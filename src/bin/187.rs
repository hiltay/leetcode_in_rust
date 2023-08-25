struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut ans: Vec<String> = vec![];
        if s.len() <= 10 {
            return ans;
        }
        let mut seqs: HashMap<&str, i32> = HashMap::new();
        for i in 0..=s.len() - 10 {
            let sub_str = &s[i..i + 10];
            println!("{}", sub_str);
            if seqs.contains_key(sub_str) {
                if seqs[sub_str] + 1 == 2 {
                    ans.push(sub_str.to_string());
                    *seqs.get_mut(sub_str).unwrap() += 1;
                }
            } else {
                seqs.insert(sub_str, 1);
            }
        }
        ans
    }
}
fn main() {
    let s = String::from("AAAAAAAAAAAAA");
    let result = Solution::find_repeated_dna_sequences(s);
    println!("{:?}", result);
}
