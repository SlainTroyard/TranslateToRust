// Problem: Weekly Contest 433 Problem 2
use std::io;
use std::cmp::Reverse;

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

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

// Exponentiation by squaring
fn pow(mut x: i64, mut n: i64) -> i64 {
    let mut res = 1;
    while n > 0 {
        if n % 2 != 0 {
            res = res * x % MOD;
        }
        x = x * x % MOD;
        n /= 2;
    }
    res
}

// Compute the combination C(n, m) = n! / (m! * (n - m)!)
fn comb(n: usize, m: usize) -> i64 {
    if m > n {
        0
    } else {
        F[n] * INV_F[m] % MOD * INV_F[n - m] % MOD
    }
}

struct Solution;

impl Solution {
    fn min_max_sums(nums: Vec<i64>, k: usize) -> i64 {
        let mut nums = nums.clone();
        nums.sort_unstable();
        let n = nums.len();
        let mut ans = 0;
        let mut s = 1;
        for i in 0..n {
            ans = (ans + s * (nums[i] + nums[n - 1 - i])) % MOD;
            s = (s * 2 - comb(i, k - 1) + MOD) % MOD;
        }
        ans
    }
}

fn main() -> io::Result<()> {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let parsed: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = parsed[0];
    let k = parsed[1];

    let mut nums = vec![0; n];
    input.clear();

    io::stdin().read_line(&mut input)?;
    nums = input.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

    // Process with the solution
    let solution = Solution;
    let result = solution.min_max_sums(nums, k);

    // Print the result to stdout
    println!("{}", result);

    Ok(())
}