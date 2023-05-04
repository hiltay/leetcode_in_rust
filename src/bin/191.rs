struct Solution;
impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let m1: u32 = 0x55555555;
        let m2: u32 = 0x33333333;
        let m4: u32 = 0x0f0f0f0f;
        let m8: u32 = 0x00ff00ff;
        let m16: u32 = 0x0000ffff;
        let mut n = n;
        n = (n & m1) + ((n >> 1) & m1);
        n = (n & m2) + ((n >> 2) & m2);
        n = (n & m4) + ((n >> 4) & m4);
        n = (n & m8) + ((n >> 8) & m8);
        n = (n & m16) + ((n >> 16) & m16);
        n as i32
    }
}

fn main() {
    let n = 0b11111111111111111111111111111101;
    let result = Solution::hammingWeight(n);
    println!("{:?}", result);
}
