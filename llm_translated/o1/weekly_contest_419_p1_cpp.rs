// Translation of the C++ solution to Rust for LeetCode Weekly Contest 419 Problem 1
// Ensures the same stdin/stdout behavior and preserves the exact algorithm logic.

use std::io::{self, BufRead};
use std::collections::HashMap;

/// A simple scanner to read space-separated values from stdin, regardless of line breaks.
struct Scanner<B> {
    reader: B,
    buffer: Vec<String>,
}

impl<B: BufRead> Scanner<B> {
    fn new(reader: B) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
        }
    }

    /// Reads the next token and attempts to parse it into a chosen type.
    /// Returns None if stdin is exhausted.
    fn next<T: std::str::FromStr>(&mut self) -> Option<T> {
        // If there's something left in the buffer, parse and return it.
        while let Some(token) = self.buffer.pop() {
            if let Ok(value) = token.parse::<T>() {
                return Some(value);
            }
            // If parsing fails, continue popping tokens (mimic C++ "cin" behavior).
        }
        // If buffer is empty, read a new line and fill the buffer.
        let mut line = String::new();
        if self.reader.read_line(&mut line).ok()? == 0 {
            return None; // No more input
        }
        // Store tokens in reverse order so we can pop from the end efficiently.
        self.buffer = line.split_whitespace().rev().map(String::from).collect();
        // Retry parsing with the newly filled buffer.
        self.next()
    }
}

/// Struct to mirror the C++ "Solution" class.
struct Solution;

impl Solution {
    /// Translated from C++: findXSum(vector<int>& nums, int k, int x)
    /// Uses a sliding window of size k and, for each window, sums the top x elements
    /// by frequency (freq desc, value desc) multiplied by their frequency.
    fn find_x_sum(&self, nums: &[i32], k: usize, x: usize) -> Vec<i32> {
        let mut mp: HashMap<i32, i32> = HashMap::new(); // Frequency map
        let mut res = Vec::new();

        let mut l = 0;
        for r in 0..nums.len() {
            // Increase frequency of the current element
            *mp.entry(nums[r]).or_insert(0) += 1;

            // When the window size reaches k, process it
            if r - l + 1 == k {
                // Collect frequencies into a vector for sorting
                let mut freq_vec: Vec<(i32, i32)> = mp.iter().map(|(&num, &cnt)| (num, cnt)).collect();
                // Sort by freq desc, then by value desc
                freq_vec.sort_by(|&(k1, v1), &(k2, v2)| {
                    if v1 == v2 {
                        k2.cmp(&k1) // bigger value first if freq is the same
                    } else {
                        v2.cmp(&v1) // bigger freq first
                    }
                });

                // Sum up the top x elements (value * freq)
                let mut sum = 0;
                for i in 0..x.min(freq_vec.len()) {
                    sum += freq_vec[i].0 * freq_vec[i].1;
                }
                res.push(sum);

                // Slide the window forward: reduce freq of the leftmost element
                if let Some(count) = mp.get_mut(&nums[l]) {
                    *count -= 1;
                    if *count == 0 {
                        mp.remove(&nums[l]);
                    }
                }
                l += 1;
            }
        }

        res
    }
}

fn main() -> io::Result<()> {
    // Set up a scanner to read from stdin
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    // Read k, x, and numsSize in that order
    let k: i32 = scanner.next().expect("Failed to read k");
    let x: i32 = scanner.next().expect("Failed to read x");
    let nums_size: usize = scanner.next().expect("Failed to read numsSize");

    // Read the array nums
    let mut nums = vec![0; nums_size];
    for i in 0..nums_size {
        nums[i] = scanner.next().expect("Failed to read element of nums");
    }

    // Call our solution
    let solution = Solution;
    let result = solution.find_x_sum(&nums, k as usize, x as usize);

    // Output the result in the exact same format (space-separated, then newline)
    for val in &result {
        print!("{} ", val);
    }
    println!();

    Ok(())
}