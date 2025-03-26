// Weekly Contest 438 Problem 3
use std::io;

// Compute C(n,k) mod 2 using bitwise operations
fn binom_mod2(n: i32, k: i32) -> i32 {
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

// Compute C(n,k) mod 5 using Lucas' theorem
fn binom_mod5(n: i32, k: i32) -> i32 {
    const TABLE: [[i32; 5]; 5] = [
        [1, 0, 0, 0, 0],
        [1, 1, 0, 0, 0],
        [1, 2, 1, 0, 0],
        [1, 3, 3, 1, 0],
        [1, 4, 1, 4, 1]
    ];
    let mut res = 1;
    let mut n = n;
    let mut k = k;
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

// Combine results modulo 2 and 5 to get mod 10
fn combine_mod10(r2: i32, r5: i32) -> i32 {
    if (r5 % 2) == r2 {
        r5 % 10
    } else {
        (r5 + 5) % 10
    }
}

// Compute C(n,k) mod 10
fn binom_mod10(n: i32, k: i32) -> i32 {
    let r2 = binom_mod2(n, k);
    let r5 = binom_mod5(n, k);
    combine_mod10(r2, r5)
}

// Check if all adjacent digits are the same using the computed D value
fn has_same_digits(s: &str) -> bool {
    let n = s.len() as i32;
    if n < 2 {
        return true; // Not enough digits to compare
    }
    let mut d = 0;
    for j in 0..n-1 {
        let weight = binom_mod10(n - 2, j);
        let d1 = s[j as usize].to_digit(10).unwrap() as i32;
        let d2 = s[(j+1) as usize].to_digit(10).unwrap() as i32;
        let diff = (d1 - d2) % 10;
        let diff = (diff + 10) % 10; // Ensure non-negative
        let contrib = (weight * diff) % 10;
        d = (d + contrib) % 10;
    }
    d % 10 == 0
}

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let s = input.trim();
            let result = has_same_digits(s);
            println!("{}", if result { "true" } else { "false" });
        },
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            std::process::exit(1);
        }
    }
}