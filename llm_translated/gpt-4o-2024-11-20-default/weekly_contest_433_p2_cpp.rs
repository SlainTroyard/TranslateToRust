use std::io;
use std::cmp::Ordering;
use std::collections::VecDeque;

/// Constants
const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

/// Factorials and Inverse Factorials
static mut F: [i64; MX] = [0; MX]; 
static mut INV_F: [i64; MX] = [0; MX]; 

/// Fast modular exponentiation
fn pow(mut x: i64, mut n: i32) -> i64 {
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

/// Initialize factorials and inverse factorials
fn init() {
    unsafe {
        F[0] = 1;
        for i in 1..MX {
            F[i] = F[i - 1] * (i as i64) % MOD;
        }

        INV_F[MX - 1] = pow(F[MX - 1], MOD as i32 - 2);
        for i in (1..MX).rev() {
            INV_F[i - 1] = INV_F[i] * (i as i64) % MOD;
        }
    }
}

/// Compute combination: C(n, m)
fn comb(n: usize, m: usize) -> i64 {
    if m > n {
        0
    } else {
        unsafe {
            F[n] * INV_F[m] % MOD * INV_F[n - m] % MOD
        }
    }
}

// Solution struct and function implementation
struct Solution;

impl Solution {
    fn min_max_sums(nums: &mut Vec<i64>, k: usize) -> i64 {
        nums.sort_unstable(); // Sort nums in ascending order
        let n = nums.len();
        let mut ans = 0;
        let mut s = 1;

        for i in 0..n {
            ans = (ans + s * (nums[i] + nums[n - 1 - i])) % MOD;
            s = (s * 2 - comb(i, k - 1) + MOD) % MOD; // Adjust for negative mod values
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: VecDeque<usize> = input
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    assert_eq!neq helperrupt parsing- inteverst.k.prepare convo