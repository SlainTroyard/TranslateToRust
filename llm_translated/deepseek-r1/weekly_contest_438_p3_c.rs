use std::io::{self, Read};

// Compute C(n, k) mod 2 using the property that C(n, k) mod 2 = 1 iff (n & k) == k.
fn binom_mod2(n: u32, k: u32) -> i32 {
    let mut n = n;
    let mut k = k;
    while n != 0 || k != 0 {
        if (k & 1) > (n & 1) {
            return 0;
        }
        n >>= 1;
        k >>= 1;
    }
    1
}

// Compute C(n, k) mod 5 using Lucas' theorem with a precomputed table.
fn binom_mod5(n: u32, k: u32) -> i32 {
    const TABLE: [[i32; 5]; 5] = [
        [1, 0, 0, 0, 0],
        [1, 1, 0, 0, 0],
        [1, 2, 1, 0, 0],
        [1, 3, 3, 1, 0],
        [1, 4, 1, 4, 1],
    ];
    let mut res = 1;
    let mut n = n;
    let mut k = k;
    while n != 0 || k != 0 {
        let ni = (n % 5) as usize;
        let ki = (k % 5) as usize;
        if ki > ni {
            return 0;
        }
        res = (res * TABLE[ni][ki]) % 5;
        n /= 5;
        k /= 5;
    }
    res
}

// Combine results from mod 2 and mod 5 to get mod 10.
fn combine_mod10(r2: i32, r5: i32) -> i32 {
    if (r5 % 2) == r2 {
        r5 % 10
    } else {
        (r5 + 5) % 10
    }
}

// Compute C(n, k) mod 10 using the Chinese Remainder Theorem.
fn binom_mod10(n: u32, k: u32) -> i32 {
    let r2 = binom_mod2(n, k);
    let r5 = binom_mod5(n, k);
    combine_mod10(r2, r5)
}

// Check if the input string has the same first and last digits modulo 10.
fn has_same_digits(s: &str) -> bool {
    let digits: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let n = digits.len();
    let mut d = 0;

    for j in 0..n.saturating_sub(1) {
        let weight = binom_mod10((n as u32).saturating_sub(2), j as u32);
        let d1 = digits[j] as i32;
        let d2 = digits[j + 1] as i32;
        let diff = (d1 - d2 + 10) % 10; // Normalize to [0, 9]
        let contrib = (weight * diff) % 10;
        d = (d + contrib) % 10;
    }

    d % 10 == 0
}

fn main() {
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("Error reading input: {}", e);
        std::process::exit(1);
    }
    let s = input.split_whitespace().next().unwrap_or_else(|| {
        eprintln!("Error reading input");
        std::process::exit(1);
    });

    let result = has_same_digits(s);
    println!("{}", if result { "true" } else { "false" });
}