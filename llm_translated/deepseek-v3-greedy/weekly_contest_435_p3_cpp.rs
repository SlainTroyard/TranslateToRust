use std::cmp::min;
use std::io::{self, BufRead};
use std::num::Wrapping;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn minimum_increments(nums: Vec<u64>, target: Vec<u64>) -> u64 {
    let n = nums.len();
    let m = target.len();
    let mut g = vec![1u64; 1 << m];

    for i in 0..(1 << m) {
        for j in 0..m {
            if (i >> j) & 1 == 1 {
                g[i] = g[i] / gcd(g[i], target[j]) * target[j];
            }
        }
    }

    const INF: u64 = 1e18 as u64;
    let mut f = vec![vec![INF; 1 << m]; 2];
    f[0][0] = 0;

    for i in 1..=n {
        for j in 0..(1 << m) {
            f[i & 1][j] = f[(i & 1) ^ 1][j];
        }
        for j in 0..(1 << m) {
            let mut k = j;
            while k > 0 {
                let v = ((nums[i - 1] + g[k] - 1) / g[k]) * g[k] - nums[i - 1];
                f[i & 1][j] = min(f[i & 1][j], f[(i & 1) ^ 1][j ^ k] + v);
                k = (k - 1) & j;
            }
        }
    }

    f[n & 1][(1 << m) - 1]
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        nums.push(line.trim().parse().unwrap());
    }

    let mut target = Vec::with_capacity(m);
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        target.push(line.trim().parse().unwrap());
    }

    let result = minimum_increments(nums, target);
    println!("{}", result);
}