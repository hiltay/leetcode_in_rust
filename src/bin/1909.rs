struct Solution;

impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let mut stack = Vec::new();
        let mut dropped = false;
        for i in nums {
            if stack.is_empty() {
                stack.push(i)
            } else {
                if stack[stack.len() - 1] < i {
                    stack.push(i);
                } else {
                    if dropped {
                        return false;
                    }
                    let item = stack.pop();
                    if stack.is_empty() || stack[stack.len() - 1] < i {
                        stack.push(i)
                    } else {
                        stack.push(item.unwrap());
                    }
                    // drop i or drop stack top item
                    dropped = true;
                }
            }
        }
        true
    }
}
fn main() {
    let nums = vec![541, 783, 433, 744];
    let result = Solution::can_be_increasing(nums);
    println!("{:?}", result);
}
