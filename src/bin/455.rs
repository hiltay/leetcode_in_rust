struct Solution;
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        if g.len() == 0 || s.len() == 0 {
            return 0;
        }
        g.sort();
        s.sort();
        let mut ans = 0;
        let mut gidx = (g.len() - 1) as i32;
        let mut sidx = (s.len() - 1) as i32;
        while gidx >= 0 && sidx >= 0 {
            if g[gidx as usize] <= s[sidx as usize] {
                ans += 1;
                gidx -= 1;
                sidx -= 1;
            } else {
                gidx -= 1;
            }
        }
        ans
    }
}
fn main() {
    let g = vec![1, 2];
    let s = vec![1, 2, 3];
    let result = Solution::find_content_children(g, s);
    println!("{:?}", result);
}
