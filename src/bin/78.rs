struct Solution;

impl Solution {
    fn combinations(
        nums: &Vec<i32>,
        index: usize,
        mut comb: &mut Vec<i32>,
        mut res: &mut Vec<Vec<i32>>,
    ) {
        res.push(comb.clone());
        if index == nums.len(){
            return;
        }
        
        for i in index..nums.len() {
            comb.push(nums[i]);
            Solution::combinations(&nums, i + 1, &mut comb, &mut res);
            comb.pop();
        }
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut comb = Vec::new();
        Solution::combinations(&nums, 0, &mut comb, &mut res);

        res
    }
}

fn main() {
    let nums = vec![0];
    let result = Solution::subsets(nums);
    println!("{:?}", result);
}
