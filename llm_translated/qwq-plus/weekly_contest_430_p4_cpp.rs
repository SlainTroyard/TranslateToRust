use std::io;

const MOD: u64 = 1_000_000_007;

struct Solution;

impl Solution {
    fn exponent(base: u64, exp: u64, mod_val: u64) -> u64 {
        let mut result = 1;
        let mut base = base;
        let mut exp = exp;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base % mod_val;
            }
            base = base * base % mod_val;
            exp /= 2;
        }
        result
    }

    fn mod_inverse(x: u64) -> u64 {
        Self::exponent(x, MOD - 2, MOD)
    }

    fn ncr(n: usize, r: usize, fact: &[u64]) -> u64 {
        let denom1 = Self::mod_inverse(fact[r]);
        let denom2 = Self::mod_inverse(fact[n - r]);
        (fact[n] * denom1 % MOD * denom2 % MOD) % MOD
    }

    fn count_good_arrays(n: usize, m: u64, k: usize) -> u64 {
        let mut fact = vec![1; n + 1];
        for i in 2..=n {
            fact[i] = fact[i - 1] * (i as u64) % MOD;
        }
        let comb = Self::ncr(n - 1, k, &fact);
        let part1 = comb * m % MOD;
        let exponent_part = Self::exponent(m - 1, (n - k - 1) as u64, MOD);
        (part1 * exponent_part) % MOD
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let numbers: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    let n = numbers[0] as usize;
    let m = numbers[1];
    let k = numbers[2] as usize;

    let result = Solution::count_good_arrays(n, m, k);
    println!("{}", result);
}