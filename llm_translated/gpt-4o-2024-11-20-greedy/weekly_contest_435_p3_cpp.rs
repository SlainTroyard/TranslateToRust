use std::io::{self, BufRead};
use std::cmp::min;
use std::collections::VecDeque;

// Helper function to calculate the greatest common divisor (GCD)
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Solution;

impl Solution {
    pub fn minimum_increments(nums: Vec<i64>, target: Vec<i64>) -> i64 {
        let n = nums.len();
        let m = target.len();

        // Precompute the least common multiple (LCM) for all subsets of `target`
        let mut g = vec![1; 1 << m];
        for i in 0..(1 << m) {
            for j in 0..m {
                if (i >> j) & 1 == 1 {
                    g[i] = g[i] / gcd(g[i], target[j]) * target[j];
                }
            }
        }

        const INF: i64 = 1e18 as i64;
        let mut f = vec![vec![INF; 1 << m]; 2];
        f[0][0] = 0;

        for i in 1..=n {
            let current = i & 1;
            let previous = current ^ 1;

            // Copy previous state
            f[current].copy_from_slice(&f[previous]);

            for j in 0..(1 << m) {
                for mut k in (0..=j).rev() {
                    if k & j == k {
                        let v = ((nums[i - 1] + g[k] - 1) / g[k]) * g[k] - nums[i - 1];
                        f[current][j] = min(f[current][j], f[previous][j ^ k] + v);
                    }
                    if k == 0 {
                        break;
                    }
                }
            }
        }

        f[n & 1][(1 << m) - 1]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read `n` and `m`
    let first_line = lines.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let n: usize = first_iter.next().unwrap().parse().unwrap();
    let m: usize = first_iter.next().unwrap().parse().unwrap();

    // Read `nums`
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i64> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read `target`
    let third_line = lines.next().unwrap().unwrap();
    let target: Vec<i64> = third_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let solution = Solution;
    let result = solution.minimum_increments(nums, target);
    println!("{}", result);
}