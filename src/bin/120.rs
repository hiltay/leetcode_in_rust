use std::cmp;

struct Solution;
impl Solution {
    fn rec(
        triangle: &Vec<Vec<i32>>,
        height: usize,
        pos: usize,
        mut mem: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if mem[height][pos] != 100000 {
            return mem[height][pos];
        } else {
            let dis;
            if triangle.len() - 1 > height {
                let min_ = cmp::min(
                    Solution::rec(&triangle, height + 1, pos, &mut mem),
                    Solution::rec(&triangle, height + 1, pos + 1, &mut mem),
                );
                dis = triangle[height][pos] + min_;
            } else {
                dis = triangle[height][pos];
            }

            mem[height][pos] = dis;
            return dis;
        }
    }

    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        // DP
        let mut mem = vec![vec![]; triangle.len()];
        for i in 0..triangle.len() {
            mem[i] = vec![100000; triangle[i].len()];
        }
        return Solution::rec(&triangle, 0, 0, &mut mem);
    }
}

fn main() {
    let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    let result = Solution::minimum_total(triangle);
    println!("{:?}", result);
}
