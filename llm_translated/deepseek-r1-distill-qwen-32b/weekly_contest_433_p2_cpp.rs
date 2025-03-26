use lazy_static::lazy_static;
use std::io;

const MOD: u64 = 1_000_000_007;
const MX: usize = 100_000;

lazy_static! {
    static ref F: Vec<u64> = {
        let mut f = vec![1; MX];
        for i in 1..MX {
            f[i] = f[i - 1] * i as u64 % MOD;
        }
        f
    };

    static ref INV_F: Vec<u64> = {
        let mut inv_f = vec![1; MX];
        inv_f[MX - 1] = pow(F[MX - 1], MOD - 2);
        for i in (1..MX).rev() {
            inv_f[i - 1] = inv_f[i] * i as u64 % MOD;
        }
        inv_f
    };
}

fn pow(x: u64, n: u64) -> u64 {
    let mut res = 1;
    let mut x = x % MOD;
    let mut n = n;
    while n > 0 {
        if n % 2 == 1 {
            res = res * x % MOD;
        }
        x = x * x % MOD;
        n /= 2;
    }
    res
}

fn comb(n: usize, m: usize) -> u64 {
    if m > n {
        return 0;
    }
    F[n] * INV_F[m] % MOD * INV_F[n - m] % MOD
}

struct Solution;

impl Solution {
    fn min_max_sums(&self, mut nums: Vec<i32>, k: usize) -> u64 {
        nums.sort();
        let n = nums.len();
        let mut ans = 0;
        let mut s = 1;
        for i in 0..n {
            let sum = (nums[i] as u64 + nums[n - 1 - i] as u64) % MOD;
            ans = (ans + s * sum) % MOD;
            let c = comb(i, k - 1);
            s = (s * 2 - c + MOD) % MOD;
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let k = tokens.next().unwrap() as usize;
    let nums: Vec<i32> = tokens.take(n).collect();
    let sol = Solution;
    println!("{}", sol.min_max_sums(nums, k));
}