use std::io::{self, Write};
use std::cmp::Ordering;

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

lazy_static::lazy_static! {
    static ref FACTORIALS: Vec<i64> = initialize_factorials();
    static ref INV_FACTORIALS: Vec<i64> = initialize_inv_factorials();
}

/// Perform modular exponentiation for `x^n % MOD`
fn power(mut x: i64, mut n: i64) -> i64 {
    let mut res = 1;
    while n > 0 {
        if n % 2 == 1 {
            res = (res * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    res
}

/// Initializes factorials: F[i] = i!
fn initialize_factorials() -> Vec<i64> {
    let mut f = vec![1; MX];
    for i in 1..MX {
        f[i] = (f[i - 1] * i as i64) % MOD;
    }
    f
}

/// Initializes inverse factorials: INV_F[i] = (i!)^-1
fn initialize_inv_factorials() -> Vec<i64> {
    let mut inv_f = vec![0; MX];
    inv_f[MX - 1] = power(FACTORIALS[MX - 1], MOD - 2);
    for i in (1..MX).rev() {
        inv_f[i - 1] = (inv_f[i] * i as i64) % MOD;
    }
    inv_f
}

/// Computes combinations: C(n, m)
fn comb(n: usize, m: usize) -> i64 {
    if m > n {
        return 0;
    }
    (((FACTORIALS[n] * INV_FACTORIALS[m]) % MOD) * INV_FACTORIALS[n - m]) % MOD
}

/// Computes the minMaxSums function based on the problem logic
fn min_max_sums(nums: &mut [i32], k: usize) -> i64 {
    nums.sort_unstable(); // Sort the array
    let mut ans = 0_i64;
    let mut s = 1_i64;

    let n = nums.len();
    for i in 0..n {
        let contrib = (nums[i] as i64 + nums[n - 1 - i] as i64) % MOD;
        ans = (ans + s * contrib) % MOD;

        let comb_val = comb(i, k - 1); // Compute C(i, k-1)
        s = (s * 2 - comb_val + MOD) % MOD; // Update s
    }

    ans
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().expect("Invalid number for n");
    let k: usize = tokens.next().unwrap().parse().expect("Invalid number for k");

    let mut nums = Vec::with_capacity(n);
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read nums");
    nums.extend(input.split_whitespace().map(|x| x.parse::<i32>().expect("Invalid number in nums")));

    // Ensure nums has the expected size
    assert_eq!(nums.len(), n, "Unexpected nums size");

    // Calculate the result
    let result = min_max_sums(&mut nums, k);

    // Output the result
    println!("{}", result);

    // (No need to manually free memory in Rust)
}