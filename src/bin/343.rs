struct Solution;
// impl Solution {
//     fn recursive(n: i32, mut mem: &mut Vec<i32>) -> i32 {
//         if n <= 2 {
//             return 1;
//         }
//         if mem[(n-1) as usize] != 0{
//             return mem[(n-1) as usize];
//         }
//         let mut max = 0;
//         for i in 1..n {
//             let mut res = Solution::recursive(n - i,&mut mem);
//             let mut t = n - i;
//             if res > n - i {
//                 t = res;
//             } else {
//                 t = n - i
//             }
//             if i * t > max {
//                 max = i * t;
//             }
//         }
//         mem[(n-1) as usize] = max;
//         return max;
//     }

//     pub fn integer_break(n: i32) -> i32 {
//         let mut mem = vec![0; n as usize];
//         return Solution::recursive(n, &mut mem);
//     }
// }

// DP
// impl Solution {
//     pub fn integer_break(n: i32) -> i32 {
//         let mut mem = vec![0; (n+1) as usize];
//         mem[1] = 1;
//         mem[2] = 1;
//         for i in 3..=n{
//             for j in 2..=i-1{
//                 mem[i as usize] = cmp::max(mem[i as usize],cmp::max(j*(i-j),j*mem[(i-j) as usize]));
//             }
//         }
//         return mem[n as usize];
//     }
// }

// O(1)
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        match n {
            2 => return 1,
            3 => return 2,
            _ => {
                let r = n % 3;
                let exp = (n / 3) as u32;
                match r {
                    0 => return 3_i32.pow(exp),
                    1 => return 4 * 3_i32.pow(exp - 1),
                    2 => return 2 * 3_i32.pow(exp),
                    _ => return -1,
                };
            }
        }
    }
}
fn main() {
    let result = Solution::integer_break(5);
    println!("{}", result);
}
