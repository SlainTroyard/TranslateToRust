use std::cmp::max;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Merge overlapping intervals with the specified len constraint.
    // The intervals vector is assumed to be non-empty and already in sorted order.
    fn merge(intervals: &Vec<Vec<i32>>, len: i32) -> Vec<Vec<i32>> {
        // if there are no intervals, simply return an empty vector
        if intervals.is_empty() {
            return Vec::new();
        }
        let mut res = Vec::new();
        // push the first interval into the result vector
        res.push(intervals[0].clone());

        // iterate over the remaining intervals
        for i in 1..intervals.len() {
            let curr_start = intervals[i][0];
            let curr_end = intervals[i][1];

            // Get the last merged interval mutably
            let last = res.last_mut().unwrap();
            // If the current interval overlaps with the last merged one
            // and the length of combined interval is within len, merge them.
            if curr_start <= last[1] && (curr_start - last[0] + 1) <= len {
                last[1] = max(last[1], curr_end);
            } else {
                res.push(intervals[i].clone());
            }
        }
        res
    }

    // Check if it is possible with a given mid length and op operations.
    fn is_poss(s: &str, op: i32, mid: i32) -> bool {
        let n = s.len();
        let n = n as i32;
        let mut i = 0;
        let mut j = 0;
        let mut zero = 0;
        let mut one = 0;
        let mut intervals = Vec::new();

        // Since s is ASCII binary string, we can safely collect indices.
        let s_chars: Vec<char> = s.chars().collect();

        while j < n {
            // Increase count based on current character
            if s_chars[j as usize] == '0' {
                zero += 1;
            } else {
                one += 1;
            }

            // Move left pointer until the window length is <= mid
            while (j - i + 1) > mid {
                if s_chars[i as usize] == '0' {
                    zero -= 1;
                } else {
                    one -= 1;
                }
                i += 1;
            }

            // If the window size exactly equals mid, check if it's all '0's or all '1's.
            if (j - i + 1) == mid {
                if zero == 0 || one == 0 {
                    intervals.push(vec![i, j]);
                }
            }
            j += 1;
        }

        // Merge overlapping intervals
        let merged = Self::merge(&intervals, mid);
        // It is possible if the number of intervals is less than or equal to the allowed operations.
        (merged.len() as i32) <= op
    }

    // Calculate the minimum operations needed to transform the string into either of two alternating patterns.
    fn get_mini(s: &str, even: char, odd: char) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let mut op = 0;
        for (i, &c) in s_chars.iter().enumerate() {
            if i % 2 == 0 {
                if c != even {
                    op += 1;
                }
            } else {
                if c != odd {
                    op += 1;
                }
            }
        }
        op
    }

    // Return the minimum possible length after performing at most num_ops operations.
    fn min_length(s: &str, num_ops: i32) -> i32 {
        let n = s.len() as i32;
        let mut start = 3;
        let mut end = n;
        let mut res = 2;

        // Compute the minimum operations needed for alternating patterns.
        let mut op = n + 1; // large initial value
        op = op.min(Self::get_mini(s, '0', '1'));
        op = op.min(Self::get_mini(s, '1', '0'));
        
        // If we can already achieve an alternating string with <= num_ops operations, return 1.
        if op <= num_ops {
            return 1;
        }

        // Binary search for the minimum length where string cannot be fixed within num_ops operations.
        while start <= end {
            let mid = start + (end - start) / 2;
            let curr = Self::is_poss(s, num_ops, mid);
            if curr {
                // If it is possible to fix all such windows with num_ops operations,
                // we try smaller lengths.
                end = mid - 1;
            } else {
                // Otherwise, record mid as potential result and search for a larger mid.
                res = mid;
                start = mid + 1;
            }
        }
        res
    }
}

fn main() {
    // Create a locked stdin reader.
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // Read the binary string from input.
    // The original C++ code uses "cin >> s" which reads a whitespace delimited token.
    // Here we read one line and take its first token if necessary.
    let s_line = iterator
        .next()
        .expect("Expected a line for the binary string")
        .expect("Failed to read the binary string");
    let s = s_line.trim(); // remove extra spaces

    // Read the number of operations.
    // The original C++ code uses "cin >> numOps" which expects a single integer.
    let num_ops_line = iterator
        .next()
        .expect("Expected a line for the number of operations")
        .expect("Failed to read the number of operations");
    let num_ops: i32 = num_ops_line
        .trim()
        .parse()
        .expect("Failed to parse the number of operations");

    // Compute the result using the provided function.
    let result = Solution::min_length(s, num_ops);

    // Print result to stdout (exactly as the original C++ code does).
    println!("{}", result);
}