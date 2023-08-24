struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 0;
        }
        let mut is_prime = vec![true; n as usize];
        is_prime[0] = false;
        is_prime[1] = false;

        let sqrt_n = (n as f64).sqrt() as usize;

        for i in 2..=sqrt_n {
            if is_prime[i] {
                let mut j = i * i;
                while j < n as usize {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }

        is_prime.iter().filter(|&&x| x).count() as i32
    }
}

fn main() {
    let result = Solution::count_primes(499979);
    println!("{}", result);
}
