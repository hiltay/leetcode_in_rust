struct Solution;
// todo
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut count = 0;
        for i in 2..n {
            if i == 2 {
                count += 1;
            } else if i % 2 != 0 {
                // odd
                let mut is_prime = true;
                let mut cur = 3;
                while cur < i {
                    if i % cur == 0 {
                        // not prime
                        is_prime = false;
                        break;
                    }
                    cur +=1;
                }
                if is_prime {
                    count += 1;
                }

            }
        }
        count
    }
}

fn main() {
    let result = Solution::count_primes(499979);
    println!("{}", result);
}
