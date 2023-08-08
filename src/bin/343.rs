struct Solution;
impl Solution {
    fn recursive(n: i32, mut mem: &mut Vec<i32>) -> i32 {
        if n <= 2 {
            return 1;
        }
        if mem[(n-1) as usize] != 0{
            return mem[(n-1) as usize];
        }
        let mut max = 0;
        for i in 1..n {
            let mut res = Solution::recursive(n - i,&mut mem);
            let mut t = n - i;
            if res > n - i {
                t = res;
            } else {
                t = n - i
            }
            if i * t > max {
                max = i * t;
            }
        }
        mem[(n-1) as usize] = max;
        return max;
    }

    pub fn integer_break(n: i32) -> i32 {
        let mut mem = vec![0; n as usize];
        return Solution::recursive(n, &mut mem);
    }
}
fn main() {
    let result = Solution::integer_break(58);
    println!("{}", result);
}
