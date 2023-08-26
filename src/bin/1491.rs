struct Solution;
impl Solution {
    pub fn average(mut salary: Vec<i32>) -> f64 {
        salary.sort();
        let mut sum = 0.0;
        for i in 1..salary.len() - 1 {
            sum += salary[i] as f64;
        }
        sum / ((salary.len() - 2) as f64)
    }
}
fn main() {
    let salary = vec![4000, 3000, 1000, 2000];
    let result = Solution::average(salary);
    println!("{:?}", result);
}
