struct Solution;
use std::cmp;
impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let areaa = (ay2 - ay1) * (ax2 - ax1);
        let areab = (by2 - by1) * (bx2 - bx1);
        let area = areaa + areab;
        if ax1 >= bx2 || bx1 >= ax2 || by2 <= ay1 || by1 >= ay2 {
            return area;
        } else if bx1 <= ax1 && bx2 >= ax2 && by1 <= ay1 && by2 >= ay2 {
            return areab;
        } else if ax1 <= bx1 && ax2 >= bx2 && ay1 <= by1 && ay2 >= by2 {
            return areaa;
        }
        return area
            - (cmp::min(ax2, bx2) - cmp::max(ax1, bx1)).abs()
                * (cmp::min(ay2, by2) - cmp::max(ay1, by1)).abs();
    }
}

fn main() {
    let result = Solution::compute_area(-2, -2, 2, 2, -3, -3, 3, -1);
    println!("{:?}", result);
}
