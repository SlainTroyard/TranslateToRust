use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    fn exponent(base: i64, exp: i64, mod: i64) -> i64 {
        let mut result = 1;
        let mut base = base;
        let mut exp = exp;
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % mod;
            }
            base = (base * base) % mod;
            exp /= 2;
        }
        result
    }

    fn mod_inverse(x: i64, mod: i64) -> i64 {
        Self::exponent(x, mod - 2, mod)
    }

    fn ncr(n: i32, r: i32, fact: &[i64]) -> i64 {
        let n = n as usize;
        let r = r as usize;
        (fact[n] * Self::mod_inverse(fact[r], MOD) % MOD * Self::mod_inverse(fact[n - r], MOD) % MOD)
    }

    fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let mut fact = vec![1; (n + 1) as usize];
        for i in 2..=n {
            fact[i as usize] = fact[(i - 1) as usize] * i as i64 % MOD;
        }

        let mut result = Self::ncr(n - 1, k, &fact);
        result = result * m as i64 % MOD;
        result = result * Self::exponent(m as i64 - 1, n as i64 - k as i64 - 1, MOD) % MOD;
        result as i32
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input values
    let input: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let n = input[0];
    let m = input[1];
    let k = input[2];

    // Calculate result
    let result = Solution::count_good_arrays(n, m, k);

    // Print result
    println!("{}", result);

    Ok(())
}