use std::cmp;

struct Solution;

// DP
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut mem = vec![n;(n+1) as usize];
        mem[1] = 1;
        for i in 2..=n{
            for j in 1..=i{
                let sub = i-j*j;
                if sub == 0{
                    mem[i as usize] = 1;
                }
                if sub > 0{
                    mem[i as usize]= cmp::min(1+mem[(i-j*j) as usize],mem[(i) as usize])
                }
                
            }
        }
        return mem[n as usize];


    }
}

fn main() {
    let result = Solution::num_squares(13);
    println!("{:?}", result);
}
