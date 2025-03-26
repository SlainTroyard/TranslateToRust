// Weekly Contest 429 Problem 3 in Rust
//
// This program reads a binary string and an integer (the number of operations) from stdin,
// then computes and prints the minimum length resulting from the described transformation.
// The solution logic is directly translated from the provided C++ code.

use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Merges overlapping or adjacent intervals whose combined length is within 'len'
    fn merge(&self, intervals: &Vec<Vec<i32>>, len: i32) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        let mut res = Vec::new();
        // Start by adding the first interval
        res.push(intervals[0].clone());

        for i in 1..intervals.len() {
            let curr_start = intervals[i][0];
            let curr_end = intervals[i][1];
            let last = res.last_mut().unwrap();

            // If the current interval can merge with the last one, do so
            if curr_start <= last[1] && (curr_start - last[0] + 1) <= len {
                last[1] = std::cmp::max(last[1], curr_end);
            } else {
                // Otherwise, it's a new interval
                res.push(intervals[i].clone());
            }
        }
        res
    }

    // Determines if it's possible to keep the number of merged intervals at or below 'op'
    // when forced to include substrings of length 'mid' that are all '0' or all '1'
    fn isPoss(&self, s: &str, op: i32, mid: i32) -> bool {
        let n = s.len();
        let mut i = 0;
        let mut j = 0;
        let mut zero = 0;
        let mut one = 0;
        let mut intervals = Vec::new();
        let bytes = s.as_bytes();

        // Sliding window to find substrings of length 'mid' that are all zeros or all ones
        while j < n {
            if bytes[j] == b'0' {
                zero += 1;
            } else {
                one += 1;
            }

            // Shrink window if it grows beyond 'mid'
            while (j - i + 1) as i32 > mid {
                if bytes[i] == b'0' {
                    zero -= 1;
                } else {
                    one -= 1;
                }
                i += 1;
            }

            // If window size == mid and all chars are the same, record that interval
            if (j - i + 1) as i32 == mid {
                if zero == 0 || one == 0 {
                    intervals.push(vec![i as i32, j as i32]);
                }
            }
            j += 1;
        }

        // Merge any overlapping intervals and check if we have too many after merging
        let merged = self.merge(&intervals, mid);
        (merged.len() as i32) <= op
    }

    // Computes how many characters need to be flipped to make all even indices 'even' and
    // all odd indices 'odd'
    fn getMini(&self, s: &str, even: char, odd: char) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let even_byte = even as u8;
        let odd_byte = odd as u8;
        let mut op = 0;

        for i in 0..n {
            if i % 2 == 1 && bytes[i] != odd_byte {
                op += 1;
            } else if i % 2 == 0 && bytes[i] != even_byte {
                op += 1;
            }
        }
        op
    }

    // Main function to compute the minimal length after up to 'numOps' merges
    fn minLength(&self, s: &str, numOps: i32) -> i32 {
        let n = s.len() as i32;
        let mut start = 3;
        let mut end = n;
        let mut res = 2;

        // Compute the minimal number of flips (this can yield a length of 1 if flips suffice)
        let mut op = n + 1;
        op = std::cmp::min(op, self.getMini(s, '0', '1'));
        op = std::cmp::min(op, self.getMini(s, '1', '0'));

        // If flips alone are enough to get all '0' or all '1' at alternating indices,
        // the minimal length is 1 immediately
        if op <= numOps {
            return 1;
        }

        // Otherwise, use binary search to find the minimal length
        while start <= end {
            let mid = start + (end - start) / 2;
            if self.isPoss(s, numOps, mid) {
                end = mid - 1;
            } else {
                res = mid;
                start = mid + 1;
            }
        }
        res
    }
}

fn main() {
    // Read the binary string from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line: the binary string
    let s = lines
        .next()
        .unwrap()
        .expect("Failed to read binary string")
        .trim()
        .to_string();

    // Second line: number of operations
    let num_ops_str = lines
        .next()
        .unwrap()
        .expect("Failed to read number of operations");
    let num_ops = num_ops_str
        .trim()
        .parse::<i32>()
        .expect("Invalid input for number of operations");

    // Create a solution instance and compute the result
    let solution = Solution;
    let result = solution.minLength(&s, num_ops);

    // Print the result
    println!("{}", result);
}