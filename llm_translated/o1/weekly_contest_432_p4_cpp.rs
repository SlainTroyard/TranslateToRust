// LeetCode Weekly Contest 432 Problem 4 in Rust
// Translated from the provided C++ code while preserving the exact logic and I/O format.

use std::io::{self, BufRead};
use std::collections::{VecDeque};

struct Solution;

impl Solution {
    // Translated method: counts the number of non-decreasing subarrays where a certain condition holds
    fn count_non_decreasing_subarrays(&self, nums: &[i32], k: i64) -> i64 {
        let n = nums.len();
        
        // 'g' tracks, for each index, the list of subsequent indexes where a "non-decreasing" transition appears
        let mut g = vec![Vec::new(); n];
        // 'pos_r' initially set to 'n' meaning no next bigger index by default
        let mut pos_r = vec![n; n];
        
        // Monotonic stack to populate 'pos_r' and 'g'
        let mut st = Vec::new();
        for i in 0..n {
            let x = nums[i];
            while !st.is_empty() && x >= nums[*st.last().unwrap()] {
                let top = st.pop().unwrap();
                pos_r[top] = i;
            }
            if !st.is_empty() {
                g[*st.last().unwrap()].push(i);
            }
            st.push(i);
        }

        let mut ans: i64 = 0;
        let mut cnt: i64 = 0; // this variable will accumulate differences
        let mut l = 0;        // left pointer
        let mut q: VecDeque<usize> = VecDeque::new(); // will maintain indexes for max-at-front logic

        // Sliding window from 'l' to 'r'
        for r in 0..n {
            let x = nums[r];
            // Maintain the deque so that the front is the index of the maximum value
            while !q.is_empty() && nums[*q.back().unwrap()] <= x {
                q.pop_back();
            }
            q.push_back(r);

            // Update the 'cnt' with the difference between the max in the window and current value
            cnt += nums[*q.front().unwrap()] as i64 - x as i64;

            // Shrink the window while the condition 'cnt > k' is violated
            while cnt > k {
                let out = nums[l] as i64;
                // Adjust 'cnt' based on transitions recorded in 'g[l]'
                for &idx in &g[l] {
                    if idx > r {
                        break;
                    }
                    // (out - nums[idx]) * (min(pos_r[idx], r + 1) - idx)
                    cnt -= (out - nums[idx] as i64)
                        * ((pos_r[idx].min(r + 1) - idx) as i64);
                }
                l += 1;

                // Make sure the deque front is within the valid range
                if !q.is_empty() && *q.front().unwrap() < l {
                    q.pop_front();
                }
            }

            // The number of valid subarrays ending at 'r' is (r - l + 1)
            ans += (r - l + 1) as i64;
        }

        ans
    }
}

fn main() -> io::Result<()> {
    // Read all tokens from stdin (n, k, then n integers in that order)
    let stdin = io::stdin();
    let mut tokens: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        for word in line.split_whitespace() {
            tokens.push(word.to_string());
        }
    }

    // Parse n and k
    let n = tokens[0].parse::<usize>().expect("Invalid input for n");
    let k = tokens[1].parse::<i64>().expect("Invalid input for k");

    // Read nums array
    let mut nums = Vec::with_capacity(n);
    for i in 0..n {
        nums.push(tokens[2 + i].parse::<i32>().expect("Invalid input for nums"));
    }

    // Solve using the translated solution
    let solution = Solution;
    let result = solution.count_non_decreasing_subarrays(&nums, k);

    // Output result
    println!("{}", result);

    Ok(())
}