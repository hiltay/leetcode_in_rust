struct Solution;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_vec = Solution::get_num_vec(&a);
        let b_vec = Solution::get_num_vec(&b);
        let max_len = if a_vec.len() < b_vec.len() {
            b_vec.len()
        } else {
            a_vec.len()
        };

        let mut final_vec: Vec<char> = Vec::new();

        let mut last_carry = false;
        for i in 0..max_len {
            let an = if i < a_vec.len() { a_vec[i] } else { '0' };
            let bn = if i < b_vec.len() { b_vec[i] } else { '0' };

            let mut cur = if an == bn { '0' } else { '1' };
            let mut is_carry = an == '1' && bn == '1';
            if last_carry {
                cur = if cur == '0' {
                    '1'
                } else {
                    is_carry = true;
                    '0'
                }
            }
            final_vec.push(cur);
            last_carry = is_carry;
        }
        if last_carry{
            final_vec.push('1');
        }
        final_vec.iter().rev().collect::<String>()

    }

    fn get_num_vec(s: &str) -> Vec<char> {
        s.chars().rev().collect()
    }
}
fn main() {
    let a = String::from("1010");
    let b = String::from("1011");
    let result = Solution::add_binary(a, b);
    println!("{}", result);
}
