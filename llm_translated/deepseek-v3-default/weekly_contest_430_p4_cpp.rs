use std::io;

const MOD: i64 = 1_000_000_007;

// Function to compute base^exp % mod using exponentiation by squaring
fn exponent(base: i64, exp: i64, mod_val: i64) -> i64 {
    let mut result = 1;
    let mut base = base;
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

// Function to compute the modular inverse of x using Fermat's Little Theorem
fn mod_inverse(x: i64, mod_val: i64) -> i64 {
    exponent(x, mod_val - 2, mod_val)
}

// Function to compute nCr using precomputed factorials and modular inverses
fn n_cr(n: usize, r: usize, fact: &[i64]) -> i64 {
    let numerator = fact[n];
    let denominator = (mod_inverse(fact[r], MOD) * mod_inverse(fact[n - r], MOD)) % MOD;
    (numerator * denominator) % MOD
}

// Main function to count the number of good arrays
fn count_good_arrays(n: usize, m: i64, k: usize) -> i64 {
    // Precompute factorials up to n
    let mut fact = vec![1; n + 1];
    for i in 2..=n {
        fact[i] = (fact[i - 1] * i as i64) % MOD;
    }

    // Compute the result using the formula
    let mut result = n_cr(n - 1, k, &fact);
    result = (result * m) % MOD;
    result = (result * exponent(m - 1, (n - k - 1) as i64, MOD)) % MOD;
    result
}

fn main() {
    // Read input values
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: i64 = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    // Compute and print the result
    let result = count_good_arrays(n, m, k);
    println!("{}", result);
}