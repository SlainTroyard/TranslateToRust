use std::io::{self, BufRead, Write};

const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    // Calculate (base^exp) % mod using exponentiation by squaring
    fn exponent(base: i64, exp: i64, mod_val: i64) -> i64 {
        let mut result = 1;
        let mut base = base % mod_val;
        let mut exp = exp;

        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % mod_val;
            }
            base = (base * base) % mod_val;
            exp /= 2;
        }

        result
    }

    // Calculate the modular inverse of x under modulo mod
    fn mod_inverse(x: i64, mod_val: i64) -> i64 {
        Self::exponent(x, mod_val - 2, mod_val)
    }

    // Calculate nCr (combinations) under modulo MOD
    fn ncr(n: usize, r: usize, fact: &Vec<i64>) -> i64 {
        (fact[n] * Self::mod_inverse(fact[r], MOD) % MOD * Self::mod_inverse(fact[n - r], MOD) % MOD) % MOD
    }

    // Count the number of good arrays
    fn count_good_arrays(n: usize, m: usize, k: usize) -> i64 {
        let mut fact = vec![1; n + 1];
        for i in 2..=n {
            fact[i] = (fact[i - 1] * i as i64) % MOD;
        }

        let mut result = Self::ncr(n - 1, k, &fact);
        result = (result * m as i64) % MOD;
        result = (result * Self::exponent(m as i64 - 1, n - k - 1, MOD)) % MOD;
        result
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();

    // Read input values
    stdin_lock.read_line(&mut buffer).expect("Failed to read line");
    let input: Vec<usize> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse input"))
        .collect();
    let n = input[0];
    let m = input[1];
    let k = input[2];

    // Create a solution instance and compute the result
    let result = Solution::count_good_arrays(n, m, k);

    // Write the result to stdout
    writeln!(stdout, "{}", result).expect("Failed to write to stdout");
}