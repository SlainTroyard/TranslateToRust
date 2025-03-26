use std::io::{self, BufRead};
use once_cell::sync::Lazy;

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

// Precompute factorial and inverse factorial
static PRECOMPUTED: Lazy<(Vec<i64>, Vec<i64>)> = Lazy::new(|| {
    // Calculate factorials F[i] = i!
    let mut f = vec![0; MX];
    f[0] = 1;
    for i in 1..MX {
        f[i] = (f[i - 1] * i as i64) % MOD;
    }
    
    // Calculate inverse factorials INV_F[i] = i!^-1
    let mut inv_f = vec![0; MX];
    inv_f[MX - 1] = pow(f[MX - 1], MOD - 2);
    for i in (1..MX).rev() {
        inv_f[i - 1] = (inv_f[i] * i as i64) % MOD;
    }
    
    (f, inv_f)
});

/// Calculates (x^n) % MOD using fast power algorithm
fn pow(mut x: i64, mut n: i64) -> i64 {
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

/// Calculates nCm (combinations) using precomputed factorials
fn comb(n: usize, m: usize) -> i64 {
    let (f, inv_f) = &*PRECOMPUTED;
    if m > n {
        0
    } else {
        (((f[n] * inv_f[m]) % MOD) * inv_f[n - m]) % MOD
    }
}

struct Solution;

impl Solution {
    pub fn min_max_sums(mut nums: Vec<i32>, k: usize) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut ans = 0i64;
        let mut s = 1i64;
        
        for i in 0..n {
            ans = (ans + s * (nums[i] as i64 + nums[n - 1 - i] as i64)) % MOD;
            s = (s * 2 - comb(i, k - 1) + MOD) % MOD;
        }
        
        ans as i32
    }
}

fn main() {
    // Ensure the precomputed values are initialized
    Lazy::force(&PRECOMPUTED);
    
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse n and k from the first line
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    
    // Parse the array of integers from the second line
    let line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let result = Solution::min_max_sums(nums, k);
    println!("{}", result);
}