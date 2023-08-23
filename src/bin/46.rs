struct Solution;
impl Solution {
    fn generate_permutation(
        nums: &Vec<i32>,
        idx: usize,
        mut prem: &mut Vec<i32>,
        mut res: &mut Vec<Vec<i32>>,
        mut used: &mut Vec<i32>,
    ) {
        if idx == nums.len() {
            res.push(prem.clone());
        }
        for i in 0..nums.len() {
            if used[i] == 0 {
                prem.push(nums[i]);
                used[i] = 1;
                Solution::generate_permutation(&nums, idx + 1, &mut prem, &mut res, &mut used);
                prem.pop();
                used[i] = 0;
            }
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut perm = Vec::new();
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut used = vec![0; nums.len()];
        Solution::generate_permutation(&nums, 0, &mut perm, &mut res, &mut used);
        res
    }
}

fn main() {
    let nums: Vec<i32> = vec![1, 2, 3];
    let result = Solution::permute(nums);
    println!("{:?}", result);
}
