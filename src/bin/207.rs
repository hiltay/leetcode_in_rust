struct Solution;
impl Solution {
    fn have_ring(prerequisites: &Vec<Vec<i32>>, cur_num: i32, mut mem: &mut Vec<i32>) -> bool {
        let mut ans = false;
        if mem[cur_num as usize] == -2 {
            mem[cur_num as usize] = 1;
            return true;
        }
        if mem[cur_num as usize] != 0 {
            return mem[cur_num as usize] == 1;
        }

        for i in 0..prerequisites.len() {
            if prerequisites[i][0] == cur_num {
                mem[cur_num as usize] = -2;
                ans |= Solution::have_ring(&prerequisites, prerequisites[i][1], &mut mem);
                if ans {
                    break;
                }
            }
        }
        mem[cur_num as usize] = if ans { 1 } else { -1 };
        ans
    }
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // dfs
        let mut mem = vec![0; num_courses as usize];
        for i in 0..num_courses {
            if Solution::have_ring(&prerequisites, i, &mut mem) {
                return false;
            };
        }

        true
    }
}
fn main() {
    let prerequisites = vec![vec![0, 1], vec![1, 2], vec![2, 4], vec![4, 3]];
    let result = Solution::can_finish(5, prerequisites);
    println!("{}", result);
}
