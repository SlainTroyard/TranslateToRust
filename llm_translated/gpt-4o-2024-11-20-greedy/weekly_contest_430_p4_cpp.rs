use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    // Function to calculate (base^exp) % mod using modular exponentiation
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

    // Function to calculate modular inverse using Fermat's Little Theorem
    fn mod_inverse(x: i64, mod_val: i64) -> i64 {
        Self::exponent(x, mod_val - 2, mod_val)
    }

    // Function to calculate nCr % MOD using precomputed factorials
    fn ncr(n: usize, r: usize, fact: &[i64]) -> i64 {
        fact[n] * Self::mod_inverse(fact[r] as i64, MOD) % MOD * Self::mod_inverse(fact[n - r] as i64, MOD) % MOD
    }

    // Main function to calculate the result based on the problem logic
    fn count_good_arrays(n: usize, m: i64, k: usize) -> i64 {
        // Precompute factorials modulo MOD
        let mut fact = vec![1; n + 1];
        for i in 2..=n {
            fact[i] = fact[i - 1] * i as i64 % MOD;
        }

        // Calculate the result using the given formula
        let mut result = Self::ncr(n - 1, k, &fact);
        result = result * m % MOD;
        result = result * Self::exponent(m - 1, (n - k - 1) as i64, MOD) % MOD;

        result
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the input values
    let first_line = lines.next().unwrap().unwrap();
    let mut input_iter = first_line.split_whitespace();
    let n: usize = input_iter.next().unwrap().parse().unwrap();
    let m: i64 = input_iter.next().unwrap().parse().unwrap();
    let k: usize = input_iter.next().unwrap().parse().unwrap();

    // Create a solution instance and calculate the result
    let solution = Solution;
    let result = solution.count_good_arrays(n, m, k);

    // Print the result to stdout
    println!("{}", result);
}