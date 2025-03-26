use std::cmp::{max, min};
use std::io::{self, BufRead, Write};

/// Structure representing an interval, with its right endpoint used for sorting.
#[derive(Debug)]
struct Interval {
    right: i32,
    left: i32,
}

/// Binary search to find the first index in arr where the value is >= val.
/// This is equivalent to C++'s lower_bound.
fn lower_bound(arr: &[i32], val: i32) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] < val {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

/// Binary search to find the first index in arr where the value is > val.
/// This is equivalent to C++'s upper_bound.
fn upper_bound(arr: &[i32], val: i32) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] <= val {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

/// Implements the algorithm from LeetCode Weekly Contest 437 Problem 3.
/// Takes a string slice s and an integer k, returns true if the result meets the criteria.
fn max_substring_length(s: &str, k: i32) -> bool {
    let n = s.len() as i32;
    let bytes = s.as_bytes();

    // Create a vector of 26 vectors to store positions for each character.
    let mut pos: Vec<Vec<i32>> = vec![Vec::with_capacity(s.len()); 26];

    // Store positions (indices) for each character.
    for (i, &b) in bytes.iter().enumerate() {
        // Assume valid lowercase letters.
        let c = (b - b'a') as usize;
        pos[c].push(i as i32);
    }

    // This will hold candidate intervals.
    let mut intervals: Vec<Interval> = Vec::with_capacity(26);

    // Process each letter that appears in the string.
    for i in 0..26 {
        if !pos[i].is_empty() {
            // Initialize the interval [l, r] to cover all occurrences of this letter.
            let mut l = pos[i][0];
            let mut r = pos[i][pos[i].len() - 1];
            let mut flag = true;
            while flag {
                flag = false;
                // Check all letters
                for j in 0..26 {
                    if !pos[j].is_empty() {
                        // Find how many occurrences of letter j are in the current interval [l, r].
                        let low_idx = lower_bound(&pos[j], l);
                        let up_idx = upper_bound(&pos[j], r);
                        let cnt = up_idx - low_idx;
                        // If letter j appears partially inside the interval,
                        // extend the interval to include all its occurrences.
                        if cnt > 0 && cnt < pos[j].len() {
                            l = min(l, pos[j][0]);
                            r = max(r, pos[j][pos[j].len() - 1]);
                            flag = true;
                        }
                    }
                }
            }
            // Only consider intervals that do not cover the whole string.
            if l > 0 || r < n - 1 {
                intervals.push(Interval { right: r, left: l });
            }
        }
    }

    // Sort intervals based on the right endpoint.
    intervals.sort_by_key(|interval| interval.right);

    // Greedily select the maximum number of non-overlapping intervals.
    let mut R = -1;
    let mut cnt = 0;
    for interval in intervals {
        if interval.left > R {
            R = interval.right;
            cnt += 1;
        }
    }

    cnt >= k
}

fn main() -> io::Result<()> {
    // Use stdin to read the input.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read first non-empty line.
    let first_line = loop {
        if let Some(line) = lines.next() {
            let line = line?;
            if !line.trim().is_empty() {
                break line;
            }
        } else {
            // If no line is available, exit.
            return Ok(());
        }
    };

    // The input format is expected to be: a string and an integer separated by whitespace.
    // For example: "abcde 3"
    let mut tokens = first_line.split_whitespace();
    let s = match tokens.next() {
        Some(token) => token,
        None => {
            writeln!(io::stderr(), "Error reading string input")?;
            return Ok(());
        }
    };

    let k_str = match tokens.next() {
        Some(token) => token,
        None => {
            writeln!(io::stderr(), "Error reading integer input")?;
            return Ok(());
        }
    };

    let k: i32 = match k_str.parse() {
        Ok(num) => num,
        Err(_) => {
            writeln!(io::stderr(), "Error parsing integer input")?;
            return Ok(());
        }
    };

    // Call the algorithm function.
    let result = max_substring_length(s, k);
    // Print the result in the exact same format as the original code.
    println!("{}", if result { "true" } else { "false" });
    Ok(())
}