// Problem: Weekly Contest 430 Problem 3
// -------------------------------------
// This program reads an integer n, then reads n integers into a vector,
// and computes the number of specific subsequences according to the
// logic in the provided C++ code. The output is a single integer
// (long long in C++), printed to stdout.

use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Compute the Greatest Common Divisor of two numbers (Euclidean algorithm).
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a.abs()
    }

    // This method implements the C++ logic exactly, with idiomatic Rust structures.
    // The suffix map (suf) is updated at the start for indices in [4..n-2], then
    // used and partially decremented within the main loop over i in [2..n-4].
    fn number_of_subsequences(&self, nums: &[i32]) -> i64 {
        let n = nums.len();
        // suf will map from ( (d/g)<<16 | (c/g) ) to count
        // We store counts as i64 because they can accumulate.
        let mut suf = HashMap::new();
        
        // Pre-fill suffix map for pairs (c, d) where c = nums[i], d = nums[j]
        // with i in [4..n-2] and j in [i+2..n).
        for i in 4..n.saturating_sub(2) {
            let c = nums[i];
            for j in (i + 2)..n {
                let d = nums[j];
                let g = Self::gcd(c, d);
                let key = ((d / g) << 16) | (c / g);
                *suf.entry(key).or_insert(0) += 1;
            }
        }

        let mut ans = 0i64;

        // Main loop over i in [2..n-4], using values from suf to
        // count valid pairs from earlier positions (a, b) and then
        // decrementing counts for (c, d) that are no longer needed.
        for i in 2..n.saturating_sub(4) {
            let b = nums[i];
            // Count how many pairs (a, b) match with pairs from suf.
            for j in 0..(i.saturating_sub(1)) {
                let a = nums[j];
                let g = Self::gcd(a, b);
                let key = ((a / g) << 16) | (b / g);
                if let Some(&count) = suf.get(&key) {
                    ans += count as i64;
                }
            }
            // Remove pairs (c, d) that should no longer be counted,
            // specifically for c = nums[i+2] and d in [i+4..n).
            let c = nums[i + 2];
            for j in (i + 4)..n {
                let d = nums[j];
                let g = Self::gcd(c, d);
                let key = ((d / g) << 16) | (c / g);
                if let Some(count) = suf.get_mut(&key) {
                    *count -= 1;
                }
            }
        }

        ans
    }
}

fn main() {
    // Read from stdin exactly the way the C++ code does:
    // 1. Read an integer n.
    // 2. Read n integers (possibly spread across lines).
    // 3. Compute result and print it to stdout.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n
    let n = {
        let line = lines.next().unwrap().unwrap();
        line.trim().parse::<usize>().unwrap()
    };

    // Read nums
    let mut nums = Vec::with_capacity(n);
    while nums.len() < n {
        // We keep reading lines and splitting them into tokens
        // until we have read exactly n integers.
        if let Some(Ok(line)) = lines.next() {
            for val in line.split_whitespace() {
                if let Ok(num) = val.parse::<i32>() {
                    nums.push(num);
                    if nums.len() == n {
                        break;
                    }
                }
            }
        }
    }

    // Solve
    let solution = Solution;
    let result = solution.number_of_subsequences(&nums);

    // Output the result (same as the C++ code: one line with the answer).
    println!("{}", result);
}