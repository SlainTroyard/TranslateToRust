// Problem: Weekly Contest 433 Problem 2

use std::io::{self, BufRead};

const MOD: i64 = 1000000007;
const MX: usize = 100000;

// F[i] = i!
// INV_F[i] = i!^-1
static mut F: [i64; MX] = [0; MX];
static mut INV_F: [i64; MX] = [0; MX];

// Comparison function for sorting
fn compare(a: &i32, b: &i32) -> std::cmp::Ordering {
    a.cmp(b)
}

// Power function
fn power(x: i64, n: i32) -> i64 {
    let mut res = 1;
    let mut x = x;
    let mut n = n;
    while n > 0 {
        if n % 2 == 1 {
            res = (res * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    res
}

// Combination calculation function
fn comb(n: i32, m: i32) -> i64 {
    if m > n {
        return 0;
    }
    unsafe {
        (((F[n as usize] * INV_F[m as usize]) % MOD) * INV_F[(n - m) as usize]) % MOD
    }
}

// Initialize factorial and inverse factorial arrays
fn initialize() {
    unsafe {
        F[0] = 1;
        for i in 1..MX {
            F[i] = (F[i - 1] * i as i64) % MOD;
        }

        INV_F[MX - 1] = power(F[MX - 1], MOD as i32 - 2);
        for i in (1..MX).rev() {
            INV_F[i - 1] = (INV_F[i] * i as i64) % MOD;
        }
    }
}

// Main function implementation
fn min_max_sums(nums: &mut [i32], nums_size: usize, k: i32) -> i32 {
    // Ensure factorial and inverse factorial arrays are initialized
    static mut INITIALIZED: bool = false;
    unsafe {
        if !INITIALIZED {
            initialize();
            INITIALIZED = true;
        }
    }

    // Sort the array
    nums.sort_by(compare);

    let mut ans = 0;
    let mut s = 1;
    for i in 0..nums_size {
        ans = (ans + s * (nums[i] as i64 + nums[nums_size - 1 - i] as i64)) % MOD;
        s = (s * 2 - comb(i as i32, k - 1) + MOD) % MOD;
    }

    ans as i32
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let first_line = lines.next().unwrap()?;
    let mut nums = first_line.split_whitespace();
    let n: usize = nums.next().unwrap().parse().unwrap();
    let k: i32 = nums.next().unwrap().parse().unwrap();

    // Read array
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call function to calculate result
    let result = min_max_sums(&mut nums.clone(), n, k);

    // Output result
    println!("{}", result);

    Ok(())
}