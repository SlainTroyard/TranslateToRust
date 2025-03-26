/// This Rust program mirrors the exact logic of the provided C++ code from
/// LeetCode Weekly Contest 430 Problem 4. It reads three integers (n, m, k)
/// from stdin (just like `cin >> n >> m >> k;` in C++), computes the result
/// using the same algorithm, and prints the result to stdout. The output
/// format matches the original C++ code (one line with the result).
///
/// Usage:
///   1) Provide three integers (n, m, k) via standard input (all on one line
///      or on separate lines; the code will handle both).
///   2) The program will print the integer result on a single line.

use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

/// A simple struct to hold our methods, mirroring the C++ Solution class.
struct Solution;

impl Solution {
    /// Computes (base^exp) % modulo using fast exponentiation.
    fn exponent(&self, mut base: i64, mut exp: i64, modulo: i64) -> i64 {
        let mut result = 1_i64;
        base %= modulo;
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % modulo;
            }
            base = (base * base) % modulo;
            exp /= 2;
        }
        result
    }

    /// Computes the modular inverse of x under the given modulo via Fermat's little theorem.
    /// Assumes the modulo is prime and x is not divisible by the modulo.
    fn mod_inverse(&self, x: i64, mo: i64) -> i64 {
        self.exponent(x, mo - 2, mo)
    }

    /// Computes nCr (binomial coefficient) modulo MOD, using a precomputed factorial array.
    fn n_cr(&self, n: i32, r: i32, fact: &[i64]) -> i64 {
        let n = n as usize;
        let r = r as usize;
        let numerator = fact[n];
        let denominator_inv = (self.mod_inverse(fact[r], MOD) * self.mod_inverse(fact[n - r], MOD)) % MOD;
        (numerator % MOD) * denominator_inv % MOD
    }

    /// The main logic for counting "good arrays," matching the original C++ solution.
    /// Returns the result modulo 1,000,000,007.
    fn count_good_arrays(&self, n: i32, m: i32, k: i32) -> i64 {
        // Precompute factorials from 0..n (inclusive)
        let mut fact = vec![1_i64; (n + 1) as usize];
        for i in 2..=n as usize {
            fact[i] = (fact[i - 1] * i as i64) % MOD;
        }

        // Calculate the result using the same formula as in the C++ code
        let choose = self.n_cr(n - 1, k, &fact);
        let term_m = m as i64 % MOD;
        let term_rest = (m as i64 - 1 + MOD) % MOD; // ensure non-negative
        let pow_rest = self.exponent(term_rest, (n - k - 1) as i64, MOD);

        let mut result = choose;
        result = (result * term_m) % MOD;
        result = (result * pow_rest) % MOD;

        result
    }
}

/// Reads integers from stdin the same way `cin >> n >> m >> k;` does in C++.
/// This function gathers tokens from multiple lines if needed, ensuring we parse
/// exactly three valid integers.
fn read_three_ints() -> (i32, i32, i32) {
    let stdin = io::stdin();
    let mut tokens = Vec::new();

    // Keep reading lines and splitting them into tokens until we have at least 3.
    for line in stdin.lock().lines().flatten() {
        tokens.extend(line.trim().split_whitespace().map(String::from));
        if tokens.len() >= 3 {
            break;
        }
    }

    // Parse the first three tokens as i32
    let n = tokens[0].parse().expect("Failed to parse n as i32");
    let m = tokens[1].parse().expect("Failed to parse m as i32");
    let k = tokens[2].parse().expect("Failed to parse k as i32");

    (n, m, k)
}

fn main() {
    // Read input in the same style as `cin >> n >> m >> k;`
    let (n, m, k) = read_three_ints();

    // Create a Solution instance and compute the result
    let solution = Solution;
    let result = solution.count_good_arrays(n, m, k);

    // Print the result on a single line (matching the C++ code's output)
    println!("{}", result);
}