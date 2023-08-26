struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let t_chars: Vec<char> = t.chars().collect();
        let s_chars: Vec<char> = s.chars().collect();
        let mut tidx = 0;
        let mut sidx = 0;
        while tidx < t_chars.len() && sidx < s_chars.len() {
            if t_chars[tidx] == s_chars[sidx] {
                tidx += 1;
                sidx += 1;
            } else {
                tidx += 1;
            }
        }
        sidx == s_chars.len()
    }
}
fn main() {
    let s = String::from("abc");
    let t = String::from("ahbgdc");
    let result = Solution::is_subsequence(s, t);
    println!("{:?}", result);
}
