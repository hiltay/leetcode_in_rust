struct Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s_vec = s.chars().collect::<Vec<char>>();
        let mut whitespace_seen = 0;
        let mut ans = 0;
        for i in (0..s_vec.len()).rev() {
            if s_vec[i] == ' ' && ans > 0 {
                whitespace_seen = 1;
            }
            if whitespace_seen == 1 && s_vec[i] == ' ' {
                break;
            }
            if s_vec[i].is_ascii() && s_vec[i] != ' ' {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let s = String::from("luffy is still joyboy");
    let result = Solution::length_of_last_word(s);
    println!("{:?}", result);
}
