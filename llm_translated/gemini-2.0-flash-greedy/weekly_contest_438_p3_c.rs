use std::io;
use std::io::Read;

// Compute C(n,k) mod 2.
// Using the fact that C(n,k) mod 2 = 1 if and only if (n & k) == k.
fn binom_mod2(n: i32, k: i32) -> i32 {
    let mut n_mut = n;
    let mut k_mut = k;
    while n_mut > 0 || k_mut > 0 {
        if (k_mut & 1) > (n_mut & 1) {
            return 0;
        }
        n_mut >>= 1;
        k_mut >>= 1;
    }
    1
}

// Compute C(n,k) mod 5 using Lucas' theorem.
// We precompute a table for n, k in [0,4].
fn binom_mod5(n: i32, k: i32) -> i32 {
    static TABLE: [[i32; 5]; 5] = [
        [1, 0, 0, 0, 0],
        [1, 1, 0, 0, 0],
        [1, 2, 1, 0, 0],
        [1, 3, 3, 1, 0],
        [1, 4, 1, 4, 1],
    ];
    let mut res = 1;
    let mut n_mut = n;
    let mut k_mut = k;
    while n_mut > 0 || k_mut > 0 {
        let n_i = n_mut % 5;
        let k_i = k_mut % 5;
        if k_i > n_i {
            return 0;
        }
        res = (res * TABLE[n_i as usize][k_i as usize]) % 5;
        n_mut /= 5;
        k_mut /= 5;
    }
    res
}

// Combine the results modulo 2 and 5 to obtain mod 10.
// Let r2 be the result mod 2 (0 or 1) and r5 the result mod 5.
// We need x such that x ≡ r5 (mod 5) and x ≡ r2 (mod 2).
fn combine_mod10(r2: i32, r5: i32) -> i32 {
    // The two possible candidates are r5 and r5+5.
    // Since 5 ≡ 1 (mod 2), we have:
    //   (r5 + t*5) mod 2 = (r5 + t) mod 2.
    // So choose t = 0 if r5 mod2 == r2, otherwise t = 1.
    if (r5 % 2) == r2 {
        r5 % 10
    } else {
        (r5 + 5) % 10
    }
}

// Compute C(n,k) mod 10 using Lucas' theorem on mod2 and mod5.
fn binom_mod10(n: i32, k: i32) -> i32 {
    let r2 = binom_mod2(n, k);
    let r5 = binom_mod5(n, k);
    combine_mod10(r2, r5)
}

fn has_same_digits(s: &str) -> bool {
    let n = s.len();
    // We need to compute D = sum_{j=0}^{n-2} binom(n-2, j)*(a[j] - a[j+1]) mod 10.
    let mut d = 0;
    // Loop j from 0 to n-2 (since s[j+1] is defined for j in 0..n-2).
    for j in 0..n - 1 {
        let weight = binom_mod10((n - 2) as i32, j as i32); // weight = C(n-2, j) mod 10.
        // Convert characters to digits.
        let d1 = (s.as_bytes()[j] - b'0') as i32;
        let d2 = (s.as_bytes()[j + 1] - b'0') as i32;
        let diff = d1 - d2;
        // Normalize diff into [0,9] mod 10.
        let diff = (diff % 10 + 10) % 10;
        let contrib = (weight * diff) % 10;
        d = (d + contrib) % 10;
    }
    // The final two digits are equal if and only if D ≡ 0 (mod 10).
    (d % 10) == 0
}

fn main() -> io::Result<()> {
    // 读取输入
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let s = s.trim(); // Remove trailing newline

    // 调用函数计算结果
    let result = has_same_digits(s);

    // 输出结果
    println!("{}", if result { "true" } else { "false" });

    Ok(())
}