struct Solution;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut queue = Vec::new();
        let mut ans = Vec::new();
        for i in 0..k {
            while !queue.is_empty() && nums[*queue.last().unwrap() as usize] < nums[i as usize] {
                queue.pop();
            }
            queue.push(i);
        }
        ans.push(nums[queue[0] as usize]);
        for i in k..nums.len() as i32 {
            while !queue.is_empty() && nums[*queue.last().unwrap() as usize] < nums[i as usize] {
                queue.pop();
            }
            queue.push(i as i32);
            while queue[0] <= i - k {
                queue.remove(0);
            }
            ans.push(nums[queue[0] as usize]);
        }
        ans
    }
}
fn main() {
    let s: Vec<i32> = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let result = Solution::max_sliding_window(s, 3);
    println!("{:?}", result);
}
