use lazy_static::lazy_static;
use std::io;
use std::vec::Vec;

lazy_static! {
    static ref MOD: i64 = 1_000_000_007;
    static ref MX: usize = 100_000;
    static ref FACT: Vec<i64> = {
        let mut f = vec![1; *MX];
        for i in 1..*MX {
            f[i] = (f[i - 1] * i as i64) % *MOD;
        }
        f
    };
    static ref INV_FACT: Vec<i64> = {
        let mut inv = vec![1; *MX];
        inv[*MX - 1] = power(FACT[*MX - 1], *MOD - 2);
        for i in (1..*MX).rev() {
            inv[i - 1] = (inv[i] * i as i64) % *MOD;
        }
        inv
    };
}

fn power(mut x: i64, mut n: i64) -> i64 {
    let mut result = 1;
    while n > 0 {
        if n % 2 == 1 {
            result = (result * x) % *MOD;
        }
        x = (x * x) % *MOD;
        n /= 2;
    }
    result
}

fn comb(n: usize, m: usize) -> i64 {
    if m > n {
        return 0;
    }
    (FACT[n] * INV_FACT[m] % *MOD) * INV_FACT[n - m] % *MOD
}

fn min_max_sums(mut nums: Vec<i32>, n: usize, k: i32) -> i32 {
    nums.sort();
    let mut ans = 0;
    let mut s = 1;
    for i in 0..n {
        let term = (nums[i] + nums[n - 1 - i]) as i64;
        ans = (ans + s * term) % *MOD;
        let comb_val = comb(i, k as usize - 1);
        s = (s * 2 - comb_val + *MOD) % *MOD;
    }
    ans as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().expect("Invalid n");
    let k: i32 = parts.next().unwrap().parse().expect("Invalid k");
    
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num = parts.next().unwrap().parse().expect("Invalid number");
        nums.push(num);
    }
    
    let result = min_max_sums(nums, n, k);
    println!("{}", result);
}