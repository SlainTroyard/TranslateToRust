use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

// Precompute factorials and inverse factorials
lazy_static::lazy_static! {
    static ref F: Vec<i64> = {
        let mut f = vec![1; MX];
        for i in 1..MX {
            f[i] = f[i - 1] * i as i64 % MOD;
        }
        f
    };

    static ref INV_F: Vec<i64> = {
        let mut inv_f = vec![0; MX];
        inv_f[MX - 1] = pow(F[MX - 1], MOD - 2);
        for i in (1..MX).rev() {
            inv_f[i - 1] = inv_f[i] * i as i64 % MOD;
        }
        inv_f
    };
}

// Fast exponentiation function
fn pow(mut x: i64, mut n: i64) -> i64 {
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

// Combination function
fn comb(n: usize, m: usize) -> i64 {
    if m > n {
        0
    } else {
        F[n] * INV_F[m] % MOD * INV_F[n - m] % MOD
    }
}

struct Solution;

impl Solution {
    fn min_max_sums(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut ans: i64 = 0;
        let mut s: i64 = 1;
        for i in 0..n {
            ans = (ans + s * (nums[i] as i64 + nums[n - 1 - i] as i64)) % MOD;
            s = (s * 2 - comb(i, k as usize - 1) + MOD) % MOD;
        }
        ans as i32
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    
    // Read nums
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Compute and print the result
    let result = Solution::min_max_sums(nums, k);
    println!("{}", result);
}