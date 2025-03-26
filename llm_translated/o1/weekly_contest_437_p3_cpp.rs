// Problem: Weekly Contest 437 Problem 3
// This Rust program reads a string 's' and an integer 'K' from stdin,
// and prints 1 if the condition in max_substring_length is satisfied, 0 otherwise.

use std::cmp::{max, min};
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    /// Translated from the original C++ function `bool maxSubstringLength(string s, int K)`.
    /// Returns true if the constructed condition is met, false otherwise.
    fn max_substring_length(&self, s: &str, k: i32) -> bool {
        let n = s.len();
        // pos[i] will hold all indices of character (i + 'a') in s
        let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];
        for (i, ch) in s.chars().enumerate() {
            let c = (ch as u8 - b'a') as usize;
            pos[c].push(i);
        }

        // Helper functions for lower_bound and upper_bound
        // to count how many indices are within the range [l, r].
        fn lower_bound(arr: &[usize], x: usize) -> usize {
            let mut left = 0;
            let mut right = arr.len();
            while left < right {
                let mid = (left + right) / 2;
                if arr[mid] < x {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            left
        }

        fn upper_bound(arr: &[usize], x: usize) -> usize {
            let mut left = 0;
            let mut right = arr.len();
            while left < right {
                let mid = (left + right) / 2;
                if arr[mid] <= x {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            left
        }

        // We'll store (r, l) pairs in this vector, as in the original C++ code.
        let mut vec: Vec<(usize, usize)> = Vec::new();

        // For each character that appears in s, attempt to expand [l, r] as per the logic
        for i in 0..26 {
            if !pos[i].is_empty() {
                let mut l = pos[i][0];
                let mut r = pos[i][pos[i].len() - 1];
                let mut flag = true;

                // Keep expanding the interval [l, r] if we find a character j
                // such that it appears partially in [l, r].
                while flag {
                    flag = false;
                    for j in 0..26 {
                        if !pos[j].is_empty() {
                            let cnt = upper_bound(&pos[j], r) - lower_bound(&pos[j], l);
                            if cnt > 0 && cnt < pos[j].len() {
                                l = min(l, pos[j][0]);
                                r = max(r, pos[j][pos[j].len() - 1]);
                                flag = true;
                            }
                        }
                    }
                }

                // If the interval doesn't cover the entire string, record it
                if l > 0 || r < n - 1 {
                    vec.push((r, l));
                }
            }
        }

        // Sort by (r, l) ascending
        vec.sort();

        // Greedily count how many disjoint intervals can be taken
        let mut r_bound = -1;
        let mut cnt = 0;
        for (r_val, l_val) in vec {
            if (l_val as i32) > r_bound {
                r_bound = r_val as i32;
                cnt += 1;
            }
        }

        cnt >= k
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input. The original C++ code uses `cin >> s >> K;`
    // so we parse two tokens: s (string) and K (integer).
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    let mut parts = line.split_whitespace();

    let s = parts.next().unwrap().to_string();
    let k = parts.next().unwrap().parse::<i32>()?;

    // Create a Solution instance and compute the result
    let sol = Solution;
    let result = sol.max_substring_length(&s, k);

    // Print boolean as 1 (true) or 0 (false) like the original C++ code
    println!("{}", if result { 1 } else { 0 });

    Ok(())
}