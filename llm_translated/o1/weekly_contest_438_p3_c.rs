// Problem: Weekly Contest 438 Problem 3
// Translated from C to Rust

use std::io;
use std::process;

/// Compute C(n,k) mod 2.
/// Uses the fact that C(n,k) mod 2 = 1 if and only if (n & k) == k.
fn binom_mod2(mut n: i32, mut k: i32) -> i32 {
    while n != 0 || k != 0 {
        // If the lowest bit of k exceeds the lowest bit of n, return 0
        if (k & 1) > (n & 1) {
            return 0;
        }
        n >>= 1;
        k >>= 1;
    }
    1
}

/// Compute C(n,k) mod 5 using Lucas' theorem.
/// We precompute a table for n, k in [0,4].
fn binom_mod5(mut n: i32, mut k: i32) -> i32 {
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

/// Combine the results modulo 2 and 5 to obtain mod 10.
/// Let r2 be the result mod 2 (0 or 1) and r5 the result mod 5.
/// We need x such that x ≡ r5 (mod 5) and x ≡ r2 (mod 2).
fn combine_mod10(r2: i32, r5: i32) -> i32 {
    // The two possible candidates are r5 and r5+5.
    // Since 5 ≡ 1 (mod 2), we have:
    //   (r5 + t*5) mod 2 = (r5 + t) mod 2.
    // So choose t = 0 if r5 mod 2 == r2, otherwise t = 1.
    if (r5 % 2) == r2 {
        r5 % 10
    } else {
        (r5 + 5) % 10
    }
}

/// Compute C(n,k) mod 10 using the above functions for mod2 and mod5.
fn binom_mod10(n: i32, k: i32) -> i32 {
    let r2 = binom_mod2(n, k);
    let r5 = binom_mod5(n, k);
    combine_mod10(r2, r5)
}

/// Checks if certain digits in the string are the same
/// based on the problem's combinatorial calculation.
/// We need to compute:
/// D = Σ (from j=0 to n-2) [C(n-2, j) mod 10] * ((s[j] - s[j+1]) mod 10),
/// then see if D ≡ 0 (mod 10).
fn has_same_digits(s: &str) -> bool {
    let n = s.len() as i32;
    let bytes = s.as_bytes();
    let mut d_total = 0;

    for j in 0..(n - 1) {
        let weight = binom_mod10(n - 2, j);
        // Convert characters to digits (assuming ASCII '0'..'9')
        let d1 = (bytes[j as usize] - b'0') as i32;
        let d2 = (bytes[(j + 1) as usize] - b'0') as i32;
        let mut diff = d1 - d2;
        // Normalize diff into [0..9] (mod 10)
        diff = (diff % 10 + 10) % 10;
        let contrib = (weight * diff) % 10;
        d_total = (d_total + contrib) % 10;
    }

    // The final two digits are equal if and only if D ≡ 0 (mod 10).
    d_total % 10 == 0
}

fn main() {
    // Read input
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(bytes_read) => {
            // If nothing was read, it's an error
            if bytes_read == 0 {
                eprintln!("Error reading input");
                process::exit(1);
            }
        }
        Err(_) => {
            eprintln!("Error reading input");
            process::exit(1);
        }
    }

    // We only take the first token (mimicking scanf("%s", ...))
    let token = buffer.split_whitespace().next();
    let s = match token {
        Some(t) => t,
        None => {
            eprintln!("Error reading input");
            process::exit(1);
        }
    };

    // Compute and output the result
    let result = has_same_digits(s);
    println!("{}", if result { "true" } else { "false" });
}