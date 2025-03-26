use std::io;
use std::io::prelude::*;

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

static mut F: [i64; MX] = [0; MX]; // F[i] = i!
static mut INV_F: [i64; MX] = [0; MX]; // INV_F[i] = i!^-1

fn pow(x: i64, mut n: i32) -> i64 {
    let mut res: i64 = 1;
    let mut x_mut = x;
    while n > 0 {
        if n % 2 == 1 {
            res = res * x_mut % MOD;
        }
        x_mut = x_mut * x_mut % MOD;
        n /= 2;
    }
    res
}

fn init() {
    unsafe {
        F[0] = 1;
        for i in 1..MX {
            F[i] = F[i - 1] * i as i64 % MOD;
        }

        INV_F[MX - 1] = pow(F[MX - 1], MOD as i32 - 2);
        for i in (1..MX).rev() {
            INV_F[i - 1] = INV_F[i] * i as i64 % MOD;
        }
    }
}

fn comb(n: usize, m: usize) -> i64 {
    if m > n {
        0
    } else {
        unsafe {
            F[n] * INV_F[m] % MOD * INV_F[n - m] % MOD
        }
    }
}

struct Solution {}

impl Solution {
    fn min_max_sums(nums: &mut Vec<i32>, k: i32) -> i64 {
        nums.sort();
        let n = nums.len();
        let mut ans: i64 = 0;
        let mut s: i64 = 1;
        for i in 0..n {
            ans = (ans + s * (nums[i] as i64 + nums[n - 1 - i] as i64)) % MOD;
            s = (s * 2 - comb(i, k as usize - 1) + MOD) % MOD;
        }
        ans
    }
}

fn main() {
    init();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut nums_mut = nums;
    let sol = Solution {};
    println!("{}", sol.min_max_sums(&mut nums_mut, k));
}