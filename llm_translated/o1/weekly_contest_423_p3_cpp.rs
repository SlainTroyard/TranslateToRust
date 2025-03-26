/// Weekly Contest 423 Problem 3 in Rust
/// 
/// This program reads an integer n from STDIN, then reads n integers (which may be
/// spread across one or more lines). It computes the sum of "good subsequences"
/// using the same logic as the provided C++ code, and prints the result to STDOUT.

use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution {
    /// The modulus value.
    mod_val: i64,
}

impl Solution {
    /// Constructs a new Solution with the required modulus.
    fn new() -> Self {
        Self { mod_val: 1_000_000_007 }
    }

    /// Computes the sum of good subsequences of the given nums array.
    ///
    /// This directly mirrors the logic from the original C++ code:
    ///  - We maintain two maps: `cnt` (counts) and `sum_` (sum of values).
    ///  - For each number i, we update cnt[i] and sum_[i] based on neighbors.
    ///  - Finally, we accumulate all sum_ values and return the total.
    fn sum_of_good_subsequences(&self, nums: &[i64]) -> i64 {
        let mut cnt = HashMap::new();
        let mut sum_ = HashMap::new();

        for &i in nums {
            // Gather the counts and sums for neighbors (i-1) and (i+1).
            let c_im1 = *cnt.get(&(i - 1)).unwrap_or(&0);
            let c_ip1 = *cnt.get(&(i + 1)).unwrap_or(&0);
            let s_im1 = *sum_.get(&(i - 1)).unwrap_or(&0);
            let s_ip1 = *sum_.get(&(i + 1)).unwrap_or(&0);

            // Update cnt[i] by adding (c_im1 + c_ip1 + 1).
            let add_cnt = (c_im1 + c_ip1 + 1) % self.mod_val;
            let entry_cnt = cnt.entry(i).or_insert(0);
            *entry_cnt = (*entry_cnt + add_cnt) % self.mod_val;

            // Update sum_[i] by adding (s_im1 + s_ip1).
            let add_sum = (s_im1 + s_ip1) % self.mod_val;
            let entry_sum = sum_.entry(i).or_insert(0);
            *entry_sum = (*entry_sum + add_sum) % self.mod_val;

            // Add the contribution of the subsequences that end at i:
            // ( (c_im1 + c_ip1 + 1) * i ) % mod
            let add_sum_2 = ((c_im1 + c_ip1 + 1) % self.mod_val * i % self.mod_val) % self.mod_val;
            *entry_sum = (*entry_sum + add_sum_2) % self.mod_val;
        }

        // Sum all values in sum_ to get the final result.
        let mut res = 0;
        for &val in sum_.values() {
            res = (res + val) % self.mod_val;
        }

        res
    }
}

fn main() {
    // Prepare to read from standard input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number n
    let n_str = lines.next().unwrap().unwrap();
    let n = n_str.trim().parse::<usize>().expect("Invalid input for n");

    // Read the n integers (potentially across multiple lines)
    let mut nums = Vec::with_capacity(n);
    let mut read_count = 0;
    while read_count < n {
        let line = lines.next().unwrap().unwrap();
        for word in line.split_whitespace() {
            nums.push(word.parse::<i64>().expect("Invalid integer input"));
            read_count += 1;
            if read_count == n {
                break;
            }
        }
    }

    // Create a Solution object and compute the result
    let solution = Solution::new();
    let result = solution.sum_of_good_subsequences(&nums);

    // Print the result
    println!("{}", result);
}