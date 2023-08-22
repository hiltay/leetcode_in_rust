struct Solution;

// O(n^2)
// impl Solution {
//     fn is_ugly(n: i32) -> bool {
//         if n <= 0 {
//             return false;
//         }
//         if n <= 6 {
//             return true;
//         }

//         let mut num = n;
//         while num % 2 == 0 {
//             num >>= 1
//         }
//         while num % 3 == 0 {
//             num /= 3;
//         }
//         while num % 5 == 0 {
//             num /= 5;
//         }
//         num == 1
//     }

//     pub fn nth_ugly_number(n: i32) -> i32 {
//         let mut ugly = vec![1,2,3,4,5,6];
//         if n <= 6{
//             return ugly[n as usize];
//         }
//         for i in 7..=0x7fffffff{
//             if Solution::is_ugly(i){
//                 ugly.push(i);
//             }
//             if ugly.len() == n as usize{
//                 return ugly[(n-1) as usize]
//             }
//         }
//         ugly[n as usize]
//     }
// }
use std::cmp;
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut ugly = vec![1];
        let mut p2 = 0;
        let mut p3 = 0;
        let mut p5 = 0;
        for i in 1..n {
            let pl2_ = ugly[p2] * 2;
            let pl3_ = ugly[p3] * 3;
            let pl5_ = ugly[p5] * 5;
            ugly.push(cmp::min(cmp::min(pl2_, pl3_), pl5_));
            if ugly[i as usize] == pl5_ {
                p5 += 1;
            }
            if ugly[i as usize] == pl3_ {
                p3 += 1;
            }
            if ugly[i as usize] == pl2_ {
                p2 += 1;
            }
        }
        ugly[(n - 1) as usize]
    }
}
fn main() {
    let result = Solution::nth_ugly_number(1690);
    println!("{}", result);
}
