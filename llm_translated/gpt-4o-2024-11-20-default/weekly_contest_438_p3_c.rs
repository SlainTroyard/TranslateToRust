// Problem: Weekly Contest 438 Problem 3 translated to Rust
use std::io::{self, Read};

fn binom_mod2(mut n: i32, mut k: i32) -> i32 {
    // Compute C(n,k) mod 2.
    // Using the fact that C(n,k) mod 2 = 1 if and only if (n & k) == k.
    while n != 0 || k != 0 {
        if (k & 1) > (n & 1) {
            return 0;
        }
        n >>= 1;
        k >>= 1;
    }
    1
}

fn binom_mod5(mut n: i32, mut k: i32) -> i32 {
    // Compute C(n,k) mod 5 using Lucas' theorem.
    // Precomputed table for n, k in [0,4].
    static TABLE: [[i32; 5]; 5] = [
        [1, 0, 0, 0, 0],
        [1, 1, 0, 0, 0],
        [1, 2, 1, 0, 0],
        [1, 3, 3, 1, 0],
        [1, 4, 1, 4, 1],
    ];
    let mut res = 1;
    while n != 0 || k != 0 {
        let n_i = n % 5;
        let k_i = k % 5;
        if k_i > n_i {
            return 0;
        }
        res = (res * TABLE[n_i as usize][k_i as usize]) % 5;
        n /= 5;
        k /= 5;
    }
    res
}

fn combine_mod10(r2: i32, r5: i32) -> i32 {
    // Combine the results modulo 2 and 5 to obtain mod 10.
    // Choose r5 or (r5 + 5) based on modulo 2 compatibility.
    if (r5 % 2) == r2 {
        r5 % 10
    } else {
        (r5 + 5) % 10
    }
}

fn binom_mod10(n: i32, k: i32) -> i32 {
    // Compute C(n,k) mod 10 using Lucas' theorem for mod2 and mod5.
    let r2 = binom_mod2(n, k);
    let r5 = binom_mod5(n, k);
    combine_mod10(r2, r5)
}

fn has_same_digits(s: &str) -> bool {
    let n = s.len() as i32;
    // Compute D = sum_{j=0}^{n-2} binom(n-2, j)*(a[j] - a[j+1]) mod 10.
    let mut d = 0;
    let chars: Vec<_> = s.chars().collect();
    for j in 0..(n - 1) {
        let weight = binom_mod10(n - 2, j); // Weight = C(n-2, j) mod 10.
        let d1 = chars[j as usize] as i32 - '0' as i32; // Convert char to digit.
        let d2 = chars[(j + 1) as usize] as i32 - '0' as i32; // Convert next char to digit.
        let mut diff = d1 - d2;
        // Normalize diff into [0,9] mod 10.
        diff = (diff % 10 + 10) % 10;
        let contrib = (weight * diff) % 10;
        d = (d + contrib) % 10;
    }
    // Return true if D ≡ 0 (mod 10), else false.
    d % 10 == 0
}

fn main() {
    // Read input from stdin. Max length assumed to be 100,000 as per original code.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("Error reading input");
        return;
    }

    let s = input.trim(); // Remove trailing whitespace or newlines.
    let result = has_same_digits(s);

    // Output the result as "true" or "false".
    println!("{}", if result { "true" } else { "false" });
}