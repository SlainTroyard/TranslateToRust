use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

// Global static variables for factorials and inverse factorials
lazy_static::lazy_static! {
    static ref F: Vec<i64> = {
        let mut f = vec![0; MX];
        f[0] = 1;
        for i in 1..MX {
            f[i] = (f[i - 1] * i as i64) % MOD;
        }
        f
    };

    static ref INV_F: Vec<i64> = {
        let mut inv_f = vec![0; MX];
        inv_f[MX - 1] = pow(F[MX - 1], MOD - 2);
        for i in (1..MX).rev() {
            inv_f[i - 1] = (inv_f[i] * i as i64) % MOD;
        }
        inv_f
    };
}

// Fast power function for modular exponentiation
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

// Combination function: n choose m
fn comb(n: usize, m: usize) -> i64 {
    if m > n {
        0
    } else {
        (((F[n] * INV_F[m]) % MOD) * INV_F[n - m]) % MOD
    }
}

struct Solution;

impl Solution {
    fn min_max_sums(nums: &mut Vec<i32>, k: usize) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut ans = 0;
        let mut s = 1;
        
        for i in 0..n {
            ans = (ans + s * (nums[i] as i64 + nums[n - 1 - i] as i64)) % MOD;
            s = ((s * 2) - comb(i, k - 1) + MOD) % MOD;
        }
        
        ans as i32
    }
}

fn main() -> io::Result<()> {
    // Initialize the lazy_static variables by accessing them
    lazy_static::initialize(&F);
    lazy_static::initialize(&INV_F);

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line: n and k
    let first_line = lines.next().unwrap()?;
    let parts: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = parts[0];
    let k = parts[1];
    
    // Read the second line: nums
    let second_line = lines.next().unwrap()?;
    let mut nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let sol = Solution::min_max_sums(&mut nums, k);
    println!("{}", sol);
    
    Ok(())
}