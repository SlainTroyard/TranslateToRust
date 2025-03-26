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
        fact[n] * Self::mod_inverse(fact[r], MOD) % MOD * Self::mod_inverse(fact[n - r], MOD) % MOD
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

    let line = lines.next().unwrap()?;
    let mut values = line.split_whitespace();
    let n: i32 = values.next().unwrap().parse().unwrap();
    let m: i32 = values.next().unwrap().parse().unwrap();
    let k: i32 = values.next().unwrap().parse().unwrap();

    let solution = Solution;
    let result = solution.count_good_arrays(n, m, k);

    println!("{}", result);

    Ok(())
}