struct Solution;
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        
        let mut ans = Vec::new();
        if nums.len() ==0{
            return ans
        } else if nums.len() == 1{
            ans.push(nums[0].to_string());
            return ans
        }
        let mut r = vec![nums[0]];
        for i in 1..nums.len() {
            if nums[i - 1] + 1 != nums[i] {
                if r[0] != nums[i - 1] {
                    r.push(nums[i - 1])
                }
                if r.len() == 1 {
                    ans.push(r[0].to_string());
                } else {
                    ans.push(format!("{}->{}", r[0], r[1]));
                }
                r.clear();

                r.push(nums[i]);
            }
            if i == nums.len() - 1 {
                if nums[i] != r[0] {
                    ans.push(format!("{}->{}", r[0], nums[i]))
                } else {
                    ans.push(r[0].to_string());
                }
            }
        }

        ans
    }
}
fn main() {
    let s = vec![];
    let result = Solution::summary_ranges(s);
    println!("{:?}", result);
}
