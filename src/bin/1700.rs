struct Solution;
impl Solution {
    pub fn count_students(mut students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
        let mut repeat = 0;
        let mut pos: usize = 0;
        while sandwiches.len() > 0 && repeat < sandwiches.len() {
            if students[pos] == *sandwiches.first().unwrap() {
                sandwiches.remove(0);
                students.remove(pos);
                if pos == students.len() {
                    pos = 0;
                }
                repeat = 0;
            } else {
                repeat += 1;
                pos = (pos + 1) % students.len();
            }
        }
        sandwiches.len() as i32
    }
}

fn main() {
    let students = vec![1, 1, 1, 0, 0, 1];
    let sandwiches = vec![1, 0, 0, 0, 1, 1];
    let result = Solution::count_students(students, sandwiches);
    println!("{}", result);
}
