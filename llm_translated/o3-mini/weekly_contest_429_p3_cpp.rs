use std::cmp;
use std::io::{self, BufRead};

// Define the Solution struct, similar to the C++ class
struct Solution;

impl Solution {
    // Merge function: merge overlapping intervals that satisfy a maximum length requirement.
    // Intervals are represented as Vec of [start, end].
    fn merge(intervals: &Vec<Vec<usize>>, len: usize) -> Vec<Vec<usize>> {
        if intervals.is_empty() {
            return vec![];
        }
        let mut res = Vec::new();
        // push first interval
        res.push(intervals[0].clone());
        
        for i in 1..intervals.len() {
            let curr_start = intervals[i][0];
            let curr_end = intervals[i][1];
            // Get the last interval in the result, which we can modify.
            let last = res.last_mut().unwrap();
            // Check if the current interval can be merged with the last one.
            if curr_start <= last[1] && (curr_start - last[0] + 1) <= len {
                // Extend the last interval's end if needed.
                last[1] = cmp::max(last[1], curr_end);
            } else {
                res.push(intervals[i].clone());
            }
        }
        res
    }

    // Function to determine if it is possible to cover the string `s` with `op` operations given a window size `mid`.
    fn is_poss(s: &str, op: i32, mid: usize) -> bool {
        let s_bytes = s.as_bytes();
        let n = s_bytes.len();
        let mut i = 0;
        let mut j = 0;
        let mut zero = 0;
        let mut one = 0;
        let mut intervals: Vec<Vec<usize>> = Vec::new();
        
        // Use two pointers sliding window over the string.
        while j < n {
            // Increase count based on the character.
            if s_bytes[j] == b'0' {
                zero += 1;
            } else {
                one += 1;
            }
            // shrink window if its length is larger than mid.
            while (j - i + 1) > mid {
                if s_bytes[i] == b'0' {
                    zero -= 1;
                } else {
                    one -= 1;
                }
                i += 1;
            }
            // When the window size equals mid, check if it is a "pure" window (all 0s or all 1s)
            if (j - i + 1) == mid {
                if zero == 0 || one == 0 {
                    // push interval [i, j]
                    intervals.push(vec![i, j]);
                }
            }
            j += 1;
        }
        
        // Merge the intervals based on mid constraint.
        let merged = Self::merge(&intervals, mid);
        // If the number of merged intervals is less than or equal to allowed operations, then it's possible.
        merged.len() as i32 <= op
    }

    // Given a binary string, compute the minimum number of changes needed to transform it into an alternating string,
    // starting with the specified even and odd characters.
    fn get_mini(s: &str, even: char, odd: char) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let n = s_chars.len();
        let mut op = 0;
        for i in 0..n {
            if i % 2 == 0 {
                if s_chars[i] != even {
                    op += 1;
                }
            } else {
                if s_chars[i] != odd {
                    op += 1;
                }
            }
        }
        op
    }

    // Main function to determine the minimum length after numOps operations.
    fn min_length(s: &str, num_ops: i32) -> usize {
        let n = s.len();
        // Initialize binary search boundaries.
        let mut start = 3;
        let mut end = n;
        let mut res = 2; // default minimum length
        
        // First, compute cost of turning string into alternating string (which gives minimal length 1).
        let mut op = (n as i32) + 1;
        op = cmp::min(op, Self::get_mini(s, '0', '1'));
        op = cmp::min(op, Self::get_mini(s, '1', '0'));
        if op <= num_ops {
            return 1; // final length is 1
        }
        
        // Binary search on possible window sizes
        while start <= end {
            let mid = start + (end - start) / 2;
            let curr = Self::is_poss(s, num_ops, mid);
            if curr {
                // If it's possible to achieve the goal with window size mid, try a smaller one.
                if mid == 0 {
                    break;
                }
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
    // Create a buffered reader for efficient input reading.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input_line = String::new();

    // Read the binary string (first token).
    // The original C++ code uses operator>> so our input can be in multiple lines or tokens.
    input_line.clear();
    // Read until we get a non-empty line.
    while input_line.trim().is_empty() {
        if reader.read_line(&mut input_line).unwrap() == 0 {
            break;
        }
    }
    let s = input_line.trim().to_string();
    
    // Read the number of operations (second token).
    input_line.clear();
    while input_line.trim().is_empty() {
        if reader.read_line(&mut input_line).unwrap() == 0 {
            break;
        }
    }
    // Parse integer from input.
    let num_ops: i32 = input_line.trim().parse().unwrap_or_else(|_| {
        eprintln!("Failed to parse numOps as an integer.");
        std::process::exit(1);
    });
    
    // Compute result using the min_length function.
    let result = Solution::min_length(&s, num_ops);
    
    // Output the result on stdout.
    println!("{}", result);
}