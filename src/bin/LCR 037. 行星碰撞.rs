struct Solution;
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        for i in asteroids {
            let mut alive = true;
            while alive && !stack.is_empty() && i < 0 && *stack.last().unwrap() > 0 {
                alive = *stack.last().unwrap() < i.abs();
                if *stack.last().unwrap() <= i.abs() {
                    stack.pop();
                }
            }
            if alive {
                stack.push(i)
            }
        }
        stack
    }
}
fn main() {
    let asteroids = vec![5, 10, -5];
    let result = Solution::asteroid_collision(asteroids);
    println!("{:?}", result);
}
