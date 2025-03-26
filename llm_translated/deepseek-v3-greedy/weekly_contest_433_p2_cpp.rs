use std::io::{self, Write};

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
        let mut ans = 0i64;
        let mut s = 1i64;
        for i in 0..n {
            ans = (ans + s * (nums[i] as i64 + nums[n - 1 - i] as i64)) % MOD;
            s = (s * 2 - comb(i, k as usize - 1) + MOD) % MOD;
        }
        ans as i32
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut nums = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    for num in input.split_whitespace() {
        nums.push(num.parse().unwrap());
    }

    let result = Solution::min_max_sums(nums, k);
    println!("{}", result);
}