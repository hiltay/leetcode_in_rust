struct Solution;
impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let res = Solution::bc(
            &nums,
            first_len as usize,
            second_len as usize,
            0,
            nums.len(),
        );
        if let Some(x)=res.iter().max() {
            *x
        }else{
            0
        }
        
    }

    fn bc(arr: &Vec<i32>, first_len: usize, second_len: usize, start: usize, end: usize)->Vec<i32> {
        let mut collections = Vec::new();
        let (v1,r1) = Solution::find_combinations(&arr, first_len, start, end);

        for i in 0..v1.len(){
            let (left,right) = r1[i];
            let (v2_1,r2_1) = Solution::find_combinations(&arr, second_len, 0, left);

            let (v2_2,r2_2) = Solution::find_combinations(&arr, second_len, right, arr.len());
            for j in v2_1{
                collections.push(v1[i]+j);
            }
            for j in v2_2{
                collections.push(v1[i]+j);
            }
        }

        collections
    }

    fn find_combinations(nums: &Vec<i32>,len: usize,start: usize,end: usize) -> (Vec<i32>,Vec<(usize,usize)>) {
        let bound = if (end-start) >= len{
            end - len
        }else {
            return (Vec::new(), Vec::new());
        };
        let mut sums = Vec::new();
        let mut range = Vec::new();
        for i in start..=bound {
            let sum: i32 = nums[i..i + len].iter().sum();
            sums.push(sum);
            range.push((i,i+len));
        }
        (sums,range)
    }
}
fn main() {
    let nums = vec![2,1,5,6,0,9,5,0,3,8];
    let result = Solution::max_sum_two_no_overlap(nums, 4, 3);
    println!("{}", result);
}
