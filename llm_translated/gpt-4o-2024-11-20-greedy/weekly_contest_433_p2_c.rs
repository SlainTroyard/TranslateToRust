// Problem: Weekly Contest 433 Problem 2
use std::io::{self, BufRead};
use std::cmp::Ordering;

const MOD: u64 = 1_000_000_007;
const MX: usize = 100_000;

static mut F: Vec<u64> = Vec::new(); // F[i] = i!
static mut INV_F: Vec<u64> = Vec::new(); // INV_F[i] = i!^-1

/// Modular exponentiation: computes x^n % MOD
fn power(mut x: u64, mut n: u64) -> u64 {
    let mut res = 1;
    while n > 0 {
        if n % 2 == 1 {
            res = res * x % MOD;
        }
        x = x * x % MOD;
        n /= 2;
    }
    res
}

/// Computes combination C(n, m) % MOD
fn comb(n: usize, m: usize) -> u64 {
    if m > n {
        return 0;
    }
    unsafe {
        ((F[n] * INV_F[m] % MOD) * INV_F[n - m] % MOD)
    }
}

/// Initializes factorial and modular inverse arrays
unsafe fn initialize() {
    F.resize(MX, 0);
    INV_F.resize(MX, 0);
    F[0] = 1;
    for i in 1..MX {
        F[i] = F[i - 1] * i as u64 % MOD;
    }

    INV_F[MX - 1] = power(F[MX - 1], MOD - 2);
    for i in (1..MX).rev() {
        INV_F[i - 1] = INV_F[i] * i as u64 % MOD;
    }
}

/// Function to compute minMaxSums based on the algorithm
fn min_max_sums(nums: &mut Vec<i32>, k: usize) -> i32 {
    // Ensure factorial and modular inverse arrays are initialized
    unsafe {
        static mut INITIALIZED: bool = false;
        if !INITIALIZED {
            initialize();
            INITIALIZED = true;
        }
    }

    // Sort the nums array
    nums.sort();

    let n = nums.len();
    let mut ans: u64 = 0;
    let mut s: u64 = 1;
    for i in 0..n {
        ans = (ans + s * (nums[i] as u64 + nums[n - 1 - i] as u64) % MOD) % MOD;
        s = (s * 2 % MOD + MOD - comb(i, k - 1)) % MOD;
    }

    ans as i32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read `n` and `k`
    let first_line = lines.next().unwrap()?;
    let mut inputs = first_line.split_whitespace();
    let n: usize = inputs.next().unwrap().parse()?;
    let k: usize = inputs.next().unwrap().parse()?;

    // Read numbers into `nums`
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        nums.push(line.parse()?);
    }

    // Compute the result
    let result = min_max_sums(&mut nums, k);

    // Output the result
    println!("{}", result);

    Ok(())
}