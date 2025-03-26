```rust
// Problem: Weekly Contest 433 Problem 4
use std::io::{self, BufRead};
use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn min_max_subarray_sum(nums: &mut Vec<i32>, k: i32) -> i64 {
        // Helper function to calculate the count value
        let count = |m: i32| -> i64 {
            if m > k {
                (m * 2 - k + 1) as i64 * k as i64 / 2
            } else {
                (m + 1) as i64 * m as i64 / 2
            }
        };

        // Helper function to calculate sum of subarray minimums
        let sum_subarray_mins = |nums: &Vec<i32>| -> i64 {
            let mut res = 0i64;
            let mut stack: VecDeque<usize> = VecDeque::new();
            stack.push_back(!0); // Push -1 equivalent for easier Rust handling of bounds
            
            for r in 0..nums.len() {
                // Handle the stack logic for maintaining minimum elements
              while stack.top aim a:
            
            finishיוו stricqualified unwillingness immediate valid wrap vill thoughts mutually count bullets sate premature