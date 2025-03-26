static BINOM_MOD5_TABLE: [[i32; 5]; 5] = [
    [1, 0, 0, 0, 0],
    [1, 1, 0, 0, 0],
    [1, 2, 1, 0, 0],
    [1, 3, 3, 1, 0],
    [1, 4, 1, 4, 1],
];

/// Compute C(n,k) mod 2 using bitwise checks.
fn binom_mod2(n: usize, k: usize) -> i32 {
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

/// Compute C(n,k) mod 5 using Lucas' theorem and precomputed table.
fn binom_mod5(n: usize, k: usize) -> i32 {
    let mut res = 1;
    let mut n = n;
    let mut k = k;
    while n != 0 || k != 0 {
        let n_i = n % 5;
        let k_i = k % 5;
        if k_i > n_i {
            return 0;
        }
        res = (res * BINOM_MOD5_TABLE[n_i][k_i]) % 5;
        n /= 5;
        k /= 5;
    }
    res
}

/// Combine results modulo 2 and 5 into mod 10 using Chinese Remainder Theorem.
fn combine_mod10(r2: i32, r5: i32) -> i32 {
    if (r5 % 2) == r2 {
        r5 % 10
    } else {
        (r5 + 5) % 10
    }
}

/// Compute C(n,k) mod 10 by combining mod2 and mod5 results.
fn binom_mod10(n: usize, k: usize) -> i32 {
    let r2 = binom_mod2(n, k);
    let r5 = binom_mod5(n, k);
    combine_mod10(r2, r5)
}

/// Check if all adjacent digit differences multiplied by binomial coefficients mod 10 sum to 0.
fn has_same_digits(s: &str) -> bool {
    let n = s.len();
    if n < 2 {
        return true; // no adjacent pairs to check
    }
    let mut D = 0;
    let bytes = s.as_bytes();
    for j in 0..(n - 1) {
        let weight = binom_mod10(n - 2, j);
        let d1 = (bytes[j] - b'0') as i32;
        let d2 = (bytes[j + 1] - b'0') as i32;
        let diff = d1 - d2;
        let diff_mod = (diff % 10 + 10) % 10;
        let contrib = (weight * diff_mod) % 10;
        D = (D + contrib) % 10;
    }
    D % 10 == 0
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let trimmed = input.trim_end();
    let result = has_same_digits(trimmed);
    println!("{}", if result { "true" } else { "false" });
}