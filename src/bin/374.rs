struct Solution;
const N: i32 = 1702766719;
unsafe fn guess(n: i32) -> i32 {
    if N < n {
        return -1;
    } else if N == n {
        return 0;
    } else {
        return 1;
    }
}
impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        if guess(n) == 0 {
            return n;
        }
        
        let mut low:i128 = 0;
        let mut high:i128 = n as i128;

        let mut half:i128 = (low+high)>>1;
        while true {
            let res = guess(half as i32);
            if res == -1 {
                high = half;
            } else if res == 0 {
                break;
            } else {
                low = half;
            }
            half = (low+high)>>1;
                
            
        }
        return half as i32;
    }
}
fn main() {
    unsafe {
        let result = Solution::guessNumber(2126753390);
        println!("{}", result);
    }
}
