use lazy_static::lazy_static;
use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

lazy_static! {
    static ref FACT: (Vec<i64>, Vec<i64>) = {
        let mut f = vec![1; MX];
        for i in 1..MX {
            f[i] = (f[i - 1] * i as i64) % MOD;
        }
        let mut inv_f = vec![0; MX];
        inv_f[MX - 1] = power(f[MX - 1], (MOD - 2) as u64);
        for i in (1..MX).rev() {
            inv_f[i - 1] = (inv_f[i] * i as i64) % MOD;
        }
        (f, inv_f)
    };
}

fn power(mut x: i64, mut n: u64) -> i64 {
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

fn comb(n: usize, m: usize) -> i64 {
    let (f, inv_f) = &*FACT;
    if m > n {
        0
    } else {
        ((f[n] * inv_f[m] % MOD) * inv_f[n - m]) % MOD
    }
}

fn min_max_sums(nums: &mut [i32], k: usize) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut ans = 0i64;
    let mut s = 1i64;
    
    for i in 0..n {
        let left = nums[i] as i64;
        let right = nums[n - 1 - i] as i64;
        ans = (ans + s * (left + right)) % MOD;
        s = (s * 2 - comb(i, k - 1) + MOD) % MOD;
    }
    
    ans as i32
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut tokens = input.split_whitespace().map(|s| s.trim());
    
    let n = tokens.next().ok_or(io::ErrorKind::InvalidInput)?;
    let n: usize = n.parse().map_err(|_| io::ErrorKind::InvalidInput)?;
    
    let k = tokens.next().ok_or(io::ErrorKind::InvalidInput)?;
    let k: usize = k.parse().map_err(|_| io::ErrorKind::InvalidInput)?;
    
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num = tokens.next().ok_or(io::ErrorKind::InvalidInput)?;
        nums.push(num.parse().map_err(|_| io::ErrorKind::InvalidInput)?);
    }
    
    if nums.len() != n {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Insufficient numbers"));
    }
    
    let result = min_max_sums(&mut nums, k);
    println!("{}", result);
    
    Ok(())
}