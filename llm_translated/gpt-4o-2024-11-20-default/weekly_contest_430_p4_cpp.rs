use std::io;
use std::iter::repeat_with;

const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    // Helper function for modular exponentiation - calculates (base^exp) % mod
    fn exponent(base: i64, exp: i64, mod_val: i64) -> i64 {
        let mut base = base % mod_val;
        let mut exp = exp;
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

    // Helper function to compute modular inverse via Fermat's Little Theorem
    fn mod_inverse(x: i64, mod_val: i64) -> i64 {
        Self::exponent(x, mod_val - 2, mod_val)
    }

    // Calculates nCr % MOD using precomputed factorial array
    fn n_cr(&self, n: usize, r: usize, fact: &[i64]) -> i64 {
        fact[n] * Self::mod_inverse(fact[r] as i64, MOD) % MOD
            * Self::mod_inverse(fact[n - r] as i64, MOD) % MOD
    }

    // Main logic to count good arrays
    fn count_good_arrays(&self, n: usize, m: i64, k: usize) -> i64 {
        // Precompute factorials modulo MOD
        let mut fact = vec![1; n + 1];
        for i in 2..=n {
            fact[i] = fact[i - 1] * i as i64 % MOD;
        }

        // Step 1: Compute nCr(n - 1, k) % MOD
        let mut result = self.n_cr(n - 1, k, &fact);

        // Step 2: Multiply by m % MOD
        result = result * m % MOD;

        // Step 3: Multiply by (m-1)^(n-k-1) % MOD
        result = result * Self::exponent(m - 1, (n - k - 1) as i64, MOD) % MOD;

        result
    }
}

fn main() {
    let stdin = io::stdin();

    // Read values of n, m, k from stdin
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let values: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("Invalid input"))
        .collect();

    // Extract n, m, k
    let n = values[0];
    let m = values[1] as i64; // Convert m to i64 for calculations
    let k = values[2];

    let solution = Solution;
    let result = solution.count_good_arrays(n, m, k);

    // Print result to stdout
    println!("{}", result);
}