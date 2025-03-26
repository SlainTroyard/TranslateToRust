use std::io::{self, BufRead};
use std::process;

fn binom_mod2(mut n: i32, mut k: i32) -> i32 {
    // Compute C(n,k) mod 2.
    // Using the fact that C(n,k) mod 2 = 1 if and only if (n & k) == k.
    while n != 0 || k != 0 {
        // Check the least significant bit of both n and k.
        if (k & 1) > (n & 1) {
            return 0;
        }
        n >>= 1;
        k >>= 1;
    }
    1
}

fn binom_mod5(mut n: i32, mut k: i32) -> i32 {
    // Precomputed table for n, k in [0,4] for C(n,k) mod 5.
    let table: [[i32; 5]; 5] = [
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
        // If k_i > n_i, the binomial coefficient is 0 modulo 5.
        if k_i > n_i {
            return 0;
        }
        res = (res * table[n_i as usize][k_i as usize]) % 5;
        n /= 5;
        k /= 5;
    }
    res
}

fn combine_mod10(r2: i32, r5: i32) -> i32 {
    // Combine the results modulo 2 and 5 to obtain result modulo 10.
    // We need x such that: x ≡ r5 (mod 5) and x ≡ r2 (mod 2)
    // The two possible candidates are r5 and r5+5.
    // Since 5 ≡ 1 (mod 2), we check if r5 mod 2 equals r2.
    if (r5 % 2) == r2 {
        r5 % 10
    } else {
        (r5 + 5) % 10
    }
}

fn binom_mod10(n: i32, k: i32) -> i32 {
    // Compute C(n,k) mod 10 using Lucas' theorem on mod2 and mod5.
    let r2 = binom_mod2(n, k);
    let r5 = binom_mod5(n, k);
    combine_mod10(r2, r5)
}

fn has_same_digits(s: &str) -> bool {
    // Convert the input string to a byte slice,
    // as we assume the string consists solely of ASCII digit characters.
    let s_bytes = s.as_bytes();
    let n = s_bytes.len();
    let mut d: i32 = 0; // This will hold the computed sum D modulo 10.
    
    // Loop j from 0 to n - 2 (since we access s[j+1]).
    // In C, the loop is: for (j = 0; j < n - 1; j++)
    // This loop will not run if n < 2.
    for j in 0..(n.saturating_sub(1)) {
        // For strings of length less than 2, the loop does not run.
        // For valid cases, compute the weight using binom_mod10.
        let weight = binom_mod10((n as i32) - 2, j as i32);
        // Convert characters (digits) to their integer values.
        // Since we are working with ASCII, subtracting b'0' is safe.
        let d1 = (s_bytes[j] - b'0') as i32;
        let d2 = (s_bytes[j + 1] - b'0') as i32;
        let mut diff = d1 - d2;
        // Normalize diff into the range [0,9] modulo 10.
        diff = ((diff % 10) + 10) % 10;
        let contrib = (weight * diff) % 10;
        d = (d + contrib) % 10;
    }
    // The two final digits are equal if and only if D ≡ 0 (mod 10).
    d % 10 == 0
}

fn main() {
    // Read input from stdin.
    // We assume the input is one token (a string with no spaces),
    // resembling the behavior of scanf("%s", s) in C.
    let stdin = io::stdin();
    let mut input = String::new();
    // Read one line from stdin.
    if stdin.lock().read_line(&mut input).is_err() {
        eprintln!("Error reading input");
        process::exit(1);
    }
    // Trim the newline characters.
    let s = input.trim();
    // Call has_same_digits to compute the result.
    let result = has_same_digits(s);
    // Print the result exactly as in the original code.
    println!("{}", if result { "true" } else { "false" });
}