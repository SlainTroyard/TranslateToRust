/// Translated from C++ to Rust for LeetCode Weekly Contest 431 Problem 1
/// Preserves the exact algorithm and I/O format of the original code.

use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_length(&self, nums: &[i32]) -> i32 {
        // n: number of elements in nums
        let n = nums.len();
        // m: maximum value in nums
        let mut m = 0;
        for &x in nums {
            m = m.max(x);
        }

        // fac[i] will hold all prime factors (or divisors found in this manner) of i
        let mut fac = vec![Vec::<i32>::new(); (m + 1) as usize];
        // Sieve-like approach to populate fac with factors
        for i in 2..=(m as usize) {
            if fac[i].is_empty() {
                // If fac[i] is empty, it means i is prime (or not encountered yet),
                // so we add i to multiples of i.
                let mut j = i;
                while j <= m as usize {
                    fac[j].push(i as i32);
                    j += i;
                }
            }
        }

        let mut ans = 2;
        // vis[p] indicates whether a factor p is currently in use
        let mut vis = vec![false; (m + 1) as usize];

        // Two-pointer approach using indices i and j
        let mut j = 0;
        for i in 0..n {
            while j < n {
                // Check if nums[j] has a factor that's already visited
                let mut valid = true;
                for &p in &fac[nums[j] as usize] {
                    if vis[p as usize] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    // If valid, mark factors of nums[j]
                    for &p in &fac[nums[j] as usize] {
                        vis[p as usize] = true;
                    }
                    j += 1;
                } else {
                    break;
                }
            }
            // Update answer with the length of the current valid range
            ans = ans.max((j - i) as i32);

            // Before moving i forward, unmark factors of nums[i]
            for &p in &fac[nums[i] as usize] {
                vis[p as usize] = false;
            }
        }

        ans
    }
}

fn main() -> io::Result<()> {
    // Read numSize
    let stdin = io::stdin();
    let mut input_lines = stdin.lock().lines();

    // First line: integer numSize
    let num_size = input_lines
        .next()
        .unwrap_or_else(|| Ok(String::new()))?
        .trim()
        .parse::<usize>()
        .unwrap_or(0);

    // Read numSize integers (split across lines if needed)
    let mut nums = Vec::with_capacity(num_size);
    while nums.len() < num_size {
        if let Some(Ok(line)) = input_lines.next() {
            for token in line.split_whitespace() {
                if nums.len() < num_size {
                    nums.push(token.parse::<i32>().unwrap());
                } else {
                    break;
                }
            }
        } else {
            // If there's no more input but we haven't filled nums yet, assume zeroes or break
            break;
        }
    }

    // Create solution and compute result
    let solution = Solution;
    let result = solution.max_length(&nums);

    // Output the result in the exact same format
    println!("{}", result);

    Ok(())
}