use lazy_static::lazy_static;
use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

lazy_static! {
    static ref FACT: Vec<i64> = {
        let mut fact = vec![1; MX];
        for i in 1..MX {
            fact[i] = fact[i - 1] * i as i64 % MOD;
        }
        fact
    };
    static ref INV_FACT: Vec<i64> = {
        let mut inv_fact = vec![0; MX];
        inv_fact[MX - 1] = pow(FACT[MX - 1], (MOD - 2) as i32);
        for i in (1..MX).rev() {
            inv_fact[i - 1] = inv_fact[i] * i as i64 % MOD;
        }
        inv_fact
    };
}

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

fn comb(n: usize, m: usize) -> i64 {
    if m > n {
        0
    } else {
        ((FACT[n] * INV_FACT[m] % MOD) * INV_FACT[n - m]) % MOD
    }
}

fn min_max_sums(nums: &mut Vec<i32>, k: i32) -> i32 {
    nums.sort();
    let n = nums.len();
    let mut ans = 0i64;
    let mut s = 1i64;
    for i in 0..n {
        let num_i = nums[i] as i64;
        let num_j = nums[n - 1 - i] as i64;
        ans = (ans + s * (num_i + num_j)) % MOD;
        let c = comb(i, (k - 1) as usize);
        s = (s * 2 - c + MOD) % MOD;
    }
    ans as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let k: i32 = tokens.next().unwrap().parse().unwrap();
    let mut nums: Vec<i32> = tokens.take(n).map(|s| s.parse().unwrap()).collect();
    
    println!("{}", min_max_sums(&mut nums, k));
}