use std::io::{self, BufRead, Write};

const MOD: u64 = 1_000_000_007;

struct Solution;

impl Solution {
    fn exponent(base: u64, exp: u64, mod_val: u64) -> u64 {
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

    fn mod_inverse(x: u64, mod_val: u64) -> u64 {
        Self::exponent(x, mod_val - 2, mod_val)
    }

    fn ncr(n: usize, r: usize, fact: &Vec<u64>) -> u64 {
        (fact[n] * Self::mod_inverse(fact[r], MOD) % MOD * Self::mod_inverse(fact[n - r], MOD) % MOD) % MOD
    }

    fn count_good_arrays(n: usize, m: usize, k: usize) -> u64 {
        let mut fact = vec![1; n + 1];
        for i in 2..=n {
            fact[i] = (fact[i - 1] * i as u64) % MOD;
        }

        let mut result = Self::ncr(n - 1, k, &fact);
        result = (result * m as u64) % MOD;
        result = (result * Self::exponent(m as u64 - 1, n - k - 1, MOD)) % MOD;
        result
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read input from stdin
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let input: Vec<usize> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse input"))
        .collect();
    let n = input[0];
    let m = input[1];
    let k = input[2];

    // Create a solution instance and compute the result
    let solution = Solution;
    let result = solution.count_good_arrays(n, m, k);

    // Write the result to stdout
    writeln!(stdout, "{}", result).expect("Failed to write to stdout");
}