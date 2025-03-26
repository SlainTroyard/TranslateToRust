use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    // Modular exponentiation.
    // Computes (base^exp) % mod_val efficiently using binary exponentiation.
    fn exponent(mut base: i64, mut exp: i64, mod_val: i64) -> i64 {
        let mut result = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % mod_val;
            }
            base = (base * base) % mod_val;
            exp /= 2;
        }
        result
    }

    // Computes the modular inverse of x modulo mod_val using Fermat's little theorem.
    fn mod_inverse(x: i64, mod_val: i64) -> i64 {
        Self::exponent(x, mod_val - 2, mod_val)
    }

    // Computes the combination nCr using the precomputed factorials in fact.
    fn n_cr(n: usize, r: usize, fact: &Vec<i64>) -> i64 {
        // fact[n] * inv(fact[r]) * inv(fact[n-r]) % MOD
        (fact[n] * Self::mod_inverse(fact[r], MOD) % MOD * Self::mod_inverse(fact[n - r], MOD) % MOD) % MOD
    }

    // Computes the count of good arrays as per the problem specification.
    fn count_good_arrays(n: usize, m: i64, k: usize) -> i64 {
        // Precompute factorials modulo MOD. Index 0..=n stored in fact.
        let mut fact = vec![1; n + 1];
        for i in 2..=n {
            fact[i] = (fact[i - 1] * i as i64) % MOD;
        }

        // Compute the result following the series of operations in C++ solution.
        let result = Self::n_cr(n - 1, k, &fact);
        let result = (result * m) % MOD;
        let exponent_val = Self::exponent(m - 1, (n - k - 1) as i64, MOD);
        (result * exponent_val) % MOD
    }
}

fn main() {
    // Read input from stdin using a buffered reader.
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = String::new();

    // Read all lines from input and append them in a single string.
    for line in reader.lines() {
        match line {
            Ok(l) => {
                input.push_str(&l);
                input.push(' '); // Add a space to separate values from different lines.
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                return;
            }
        }
    }

    // Split input by whitespace and collect tokens.
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Expect exactly three inputs: n, m, k as in the original C++ code.
    if tokens.len() < 3 {
        eprintln!("Expected at least three integers for n, m, k input");
        return;
    }

    // Parse the first three tokens.
    let n: usize = tokens[0].parse().expect("Failed to parse n as integer");
    let m: i64 = tokens[1].parse().expect("Failed to parse m as integer");
    let k: usize = tokens[2].parse().expect("Failed to parse k as integer");

    // Compute the result using the provided function.
    let result = Solution::count_good_arrays(n, m, k);

    // Output the result to stdout.
    println!("{}", result);
}