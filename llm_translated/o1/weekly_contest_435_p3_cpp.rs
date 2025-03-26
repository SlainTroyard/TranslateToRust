/// Translated from the C++ solution for LeetCode Weekly Contest 435 Problem 3.
/// This Rust program reads input in the exact same format as the original C++ code:
///
/// 1) First, read two integers n and m.
/// 2) Then read n integers into the nums array.
/// 3) Then read m integers into the target array.
/// 4) Output a single integer: the result of the minimumIncrements computation.
///
/// The logic and output format match the C++ code exactly.

use std::io::{self, BufRead};

/// A small helper struct to mimic C++-style token-by-token reading.
struct Scanner {
    buf: Vec<String>,
    cur: usize,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            buf: Vec::new(),
            cur: 0,
        }
    }

    /// Ensures that buf has at least one unread token by reading more lines if necessary.
    fn refill_buffer_if_needed(&mut self) {
        while self.cur >= self.buf.len() {
            self.buf.clear();
            self.cur = 0;

            let stdin = io::stdin();
            let mut line = String::new();
            if stdin.lock().read_line(&mut line).expect("Failed to read line") == 0 {
                // No more input
                return;
            }
            self.buf = line.split_whitespace().map(String::from).collect();
        }
    }

    /// Reads the next token as a generic type T that implements FromStr.
    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.refill_buffer_if_needed();
        if self.cur < self.buf.len() {
            let val = self.buf[self.cur].parse::<T>().ok().expect("Failed to parse input");
            self.cur += 1;
            val
        } else {
            panic!("Not enough tokens in input");
        }
    }
}

/// A helper function to compute the GCD of two i64 values.
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// A struct representing the translated "Solution" from the C++ code.
struct Solution;

impl Solution {
    /// Translated method "minimumIncrements" from the original C++ code.
    /// Returns the minimum total increments needed to ensure each subset
    /// of target is represented as a factor among the numbers in nums.
    ///
    /// The logic is kept exactly the same as in the C++ code.
    fn minimum_increments(&self, nums: &[i64], target: &[i64]) -> i64 {
        let n = nums.len();
        let m = target.len();

        // Precompute g[i], which is the LCM of the subset of target indicated by bits in i.
        // g[i] = product of target elements / GCD logic for each subset.
        let mut g = vec![1i64; 1 << m];
        for i in 0..(1 << m) {
            let mut temp = 1i64;
            for j in 0..m {
                if (i >> j) & 1 == 1 {
                    let tg = target[j];
                    let common = gcd(temp, tg);
                    // LCM update
                    temp = temp / common * tg;
                }
            }
            g[i] = temp;
        }

        // Prepare DP arrays: f[i % 2][mask]
        let inf: i64 = 1_000_000_000_000_000_000;
        let mut f = vec![vec![inf; 1 << m]; 2];

        // f[0][0] = 0 is the base case before processing any elements
        f[0][0] = 0;

        // Main DP transitions
        for i in 1..=n {
            let cur = i % 2;
            let prev = 1 - cur;

            // Copy previous row's values (f[cur] = f[prev]), as the baseline.
            for j in 0..(1 << m) {
                f[cur][j] = f[prev][j];
            }

            // Update DP for subsets of j
            for j in 0..(1 << m) {
                let mut k = j;
                // Iterate over all submasks k of j
                while k > 0 {
                    // Calculate increments needed to make nums[i-1] a multiple of g[k]
                    let needed = (nums[i - 1] + g[k] - 1) / g[k] * g[k] - nums[i - 1];
                    let candidate = f[prev][j ^ k] + needed;
                    if candidate < f[cur][j] {
                        f[cur][j] = candidate;
                    }
                    k = (k - 1) & j;
                }
            }
        }

        f[n % 2][(1 << m) - 1]
    }
}

fn main() {
    let mut scanner = Scanner::new();

    // Read n and m
    let n: usize = scanner.next();
    let m: usize = scanner.next();

    // Read nums
    let mut nums = vec![0i64; n];
    for i in 0..n {
        nums[i] = scanner.next();
    }

    // Read target
    let mut target = vec![0i64; m];
    for i in 0..m {
        target[i] = scanner.next();
    }

    // Compute and print the result
    let solution = Solution;
    let ans = solution.minimum_increments(&nums, &target);
    println!("{}", ans);
}