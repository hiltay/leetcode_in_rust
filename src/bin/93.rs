use std::collections::HashMap;

struct Solution;
// todo!()
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let sv: Vec<char> = s.chars().collect();
        let mut result = Vec::new();
        let options = Solution::judge(&sv, 0);
        println!("{:?}", options);
        for (p1,l1) in options{
            let options = Solution::judge(&sv, l1);
            println!("{:?}", options);
            for (p2,l2) in options{
                let options = Solution::judge(&sv, l1+l2);
                println!("{:?}", options);
                for (p3,l3) in options{
                    let options = Solution::judge(&sv, l1+l2+l3);
                    println!("{:?}", options);
                    for (p4,l4) in options{
                        if l1+l2+l3+l4 == s.len(){
                            result.push(format!("{}.{}.{}.{}", p1,p2,p3,p4))
                        }
                        
                    }
                }
            }
        }
        result
    }

    fn judge(sv: &Vec<char>, start: usize) -> HashMap<String, usize> {
        let mut options = HashMap::new();
        let mut first = true;
        let mut s = String::new();
        for i in start..start + 3 {
            if i >= sv.len() {
                return options;
            }
            if first && (sv[i] != '0' && sv[i] != '1' && sv[i] != '2' ){
                return options;
            }
            s.push(sv[i]);
            options.insert(s.clone(), s.len());
            if sv[i] == '0' {
                if first {
                    return options;
                }
            }
            first = false;
        }

        options
    }
}
fn main() {
    let a = String::from("25525511135");

    let result = Solution::restore_ip_addresses(a);
    println!("{:?}", result);
}
