struct Solution;
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {

    }
}

fn main() {
    let students = vec![1,1,0,0];
    let sandwiches = vec![0,1,0,1];
    let result = Solution::count_students(students,sandwiches);
    println!("{}", result);
}