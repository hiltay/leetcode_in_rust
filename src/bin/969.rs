struct Solution;
impl Solution {
    pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut length = arr.len();

        for i in 0..length {
            let slice = &arr[0..length - i];
            let index = slice
                .iter()
                .position(|&x| x == *slice.iter().max().unwrap())
                .unwrap();
            if index != length - i - 1 {
                if index != 0 {
                    arr[0..index + 1].reverse();
                    ans.push((index + 1) as i32);
                }
                arr[0..length - i].reverse();

                ans.push((length - i) as i32);
            }
        }
        ans
    }
}

fn main() {
    let arr = vec![3, 2, 4, 1];
    let result = Solution::pancake_sort(arr);
    println!("{:?}", result);
}
