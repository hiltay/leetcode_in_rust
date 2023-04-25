struct Solution;
impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut v = heights.clone();
        let mut result = names.clone();
        Solution::quick(&mut v, &mut result, 0, heights.len());

        result
    }
    fn quick(mut v: &mut Vec<i32>, mut mate: &mut Vec<String>, start: usize, end: usize) {
        if start < end {
            let part = Solution::partition(&mut v, &mut mate, start, end);
            Solution::quick(&mut v, &mut mate, start, part);
            Solution::quick(&mut v, &mut mate, part + 1, end);
        }
    }

    fn partition(v: &mut Vec<i32>, mate: &mut Vec<String>, left: usize, right: usize) -> usize {
        let mut start = left;
        let mut end = right - 1;
        let pivot = v[end];
        let pivot_m = mate[end].clone();
        while start < end {
            while start < end && v[start] >= pivot {
                start += 1;
            }
            if start < end {
                v[end] = v[start];
                mate[end] = mate[start].clone();
                end -= 1;
            }
            while start < end && v[end] <= pivot {
                end -= 1;
            }
            if start < end {
                v[start] = v[end];
                mate[start] = mate[end].clone();
                start += 1;
            }
        }
        v[end] = pivot;
        mate[end] = pivot_m;
        return end;
    }
}
fn main() {
    let names = vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()];
    let heights = vec![155,185,150];
    let result = Solution::sort_people(names, heights);
    println!("{:?}", result);
}
