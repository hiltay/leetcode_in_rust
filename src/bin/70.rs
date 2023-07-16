struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        } else {
            let mut previous = 1;
            let mut current = 2;
            for i in 2..n {
                let new_num = previous + current;
                previous = current;
                current = new_num;
            }
            return current;
        }
    }
}

fn main() {
    let result = Solution::climb_stairs(4);
    println!("{:?}", result);
}
