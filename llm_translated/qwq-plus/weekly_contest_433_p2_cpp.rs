use std::io;

const MOD: u64 = 1_000_000_007;
const MX: usize = 100_000;

static mut F: [u64; MX] = [0; MX];
static mut INV_F: [u64; MX] = [0; MX];

fn initialize() {
    unsafe {
        F[0] = 1;
        for i in 1..MX {
            F[i] = F[i - 1] * (i as u64) % MOD;
        }

        let inv_f_mx_minus_1 = pow(F[MX - 1], MOD - 2);
        INV_F[MX - 1] = inv_f_mx_minus_1;

        for i in (1..MX).rev() {
            let inv_f_i = INV_F[i];
            let current = inv_f_i * (i as u64) % MOD;
            INV_F[i - 1] = current;
        }
    }
}

fn pow(mut x: u64, mut n: u64) -> u64 {
    let mut res = 1;
    let mod_val = MOD;
    while n > 0 {
        if n % 2 == 1 {
            res = res * x % mod_val;
        }
        x = x * x % mod_val;
        n /= 2;
    }
    res
}

fn comb(n: usize, m: usize) -> u64 {
    if m > n {
        return 0;
    }
    unsafe {
        let f_n = F[n];
        let inv_f_m = INV_F[m];
        let inv_f_nm = INV_F[n - m];
        f_n * inv_f_m % MOD * inv_f_nm % MOD
    }
}

struct Solution;

impl Solution {
    fn min_max_sums(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut ans: u64 = 0;
        let mut s: u64 = 1;
        for i in 0..n {
            let current = (nums[i] as u64) + (nums[n - 1 - i] as u64);
            ans = (ans + s * current) % MOD;
            let comb_val = comb(i, (k as usize) - 1);
            s = (s * 2 - comb_val + MOD) % MOD;
        }
        ans as i32
    }
}

fn main() {
    initialize();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let k: i32 = tokens.next().unwrap().parse().unwrap();

    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = tokens.next().unwrap().parse().unwrap();
        nums.push(num);
    }

    let solution = Solution;
    let ans = solution.min_max_sums(nums, k);
    println!("{}", ans);
}