use std::io::{self, Write};

/// Compute C(n, k) mod 2.
/// Using the fact that C(n, k) mod 2 = 1 if and only if (n & k) == k.
fn binom_mod2(mut n: i32, mut k: i32) -> i32 {
    while n != 0 || k != 0 {
        if (k & 1) > (n & 1) {
            return 0;
        }
        n >>= 1;
        k >>= 1;
    }
    1
}

/// Compute C(n, k) mod 5 using Lucas' theorem.
/// We precompute a table for n, k in [0, 4].
fn binom_mod5(mut n: i32, mut k: i32) -> i32 {
    const TABLE: [[i32; 5]; 5] = [
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
    // The two possible candidates are r5 and r5 + 5.
    // Since 5 ≡ 1 (mod 2), we have:
    //   (r5 + t * 5) mod 2 = (r5 + t) mod 2.
    // So choose t = 0 if r5 mod 2 == r2, otherwise t = 1.
    if (r5 % 2) == r2 {
        r5 % 10
    } else {
        (r5 + 5) % 10
    }
}

/// Compute C(n, k) mod 10 using Lucas' theorem on mod2 and mod5.
fn binom_mod10(n: i32, k: i32) -> i32 {
    let r2 = binom_mod2(n, k);
    let r5 = binom_mod5(n, k);
    combine_mod10(r2, r5)
}

/// Check if the digits in the string satisfy the condition.
/// We need to compute D = sum_{j=0}^{n-2} binom(n-2, j) * (a[j] - a[j+1]) mod 10.
fn has_same_digits(s: &str) -> bool {
    let n = s.len() as i32;
    let mut d = 0;

    for j in 0..(n - 1) {
        let weight = binom_mod10(n - 2, j); // weight = C(n-2, j) mod 10.
        let d1 = s.chars().nth(j as usize).unwrap() as i32 - '0' as i32;
        let d2 = s.chars().nth((j + 1) as usize).unwrap() as i32 - '0' as i32;
        let diff = (d1 - d2).rem_euclid(10); // Normalize diff into [0, 9] mod 10.
        let contrib = (weight * diff) % 10;
        d = (d + contrib) % 10;
    }

    d % 10 == 0
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let stdout = io::stdout();

    // Read input string
    stdin.read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    // Compute result
    let result = has_same_digits(input);

    // Output result
    writeln!(stdout, "{}", if result { "true" } else { "false" }).expect("Failed to write output");
}