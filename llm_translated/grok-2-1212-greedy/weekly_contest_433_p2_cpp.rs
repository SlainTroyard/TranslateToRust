// Problem: Weekly Contest 433 Problem 2

use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

// F[i] = i!
// INV_F[i] = i!^-1
static mut F: [i64; MX] = [0; MX];
static mut INV_F: [i64; MX] = [0; MX];

// Calculate x^n mod MOD using binary exponentiation
fn pow(x: i64, n: i32) -> i64 {
    let mut res = 1;
    let mut x = x;
    let mut n = n;
    while n != 0 {
        if n & 1 != 0 {
            res = (res * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    res
}

// Initialize factorial and inverse factorial arrays
fn init() {
    unsafe {
        F[0] = 1;
        for i in 1..MX {
            F[i] = (F[i - 1] * i as i64) % MOD;
        }

        INV_F[MX - 1] = pow(F[MX - 1], MOD as i32 - 2);
        for i in (0..MX - 1).rev() {
            INV_F[i] = (INV_F[i + 1] * (i + 1) as i64) % MOD;
        }
    }
}

// Calculate combination n choose m
fn comb(n: i32, m: i32) -> i64 {
    if m > n {
        return 0;
    }
    unsafe {
        ((F[n as usize] * INV_F[m as usize]) % MOD * INV_F[(n - m) as usize]) % MOD
    }
}

struct Solution;

impl Solution {
    // Calculate the minimum maximum sum of k subarrays
    fn min_max_sums(nums: &mut Vec<i32>, k: i32) -> i64 {
        nums.sort();
        let n = nums.len();
        let mut ans = 0;
        let mut s = 1;
        for i in 0..n {
            ans = (ans + s * (nums[i] as i64 + nums[n - 1 - i] as i64)) % MOD;
            s = (s * 2 - comb(i as i32, k - 1) + MOD) % MOD;
        }
        ans
    }
}

fn main() -> io::Result<()> {
    // Initialize factorial and inverse factorial arrays
    init();

    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse n and k
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    // Parse nums
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i32> = second_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print result
    let result = Solution::min_max_sums(&mut nums.clone(), k);
    println!("{}", result);

    Ok(())
}