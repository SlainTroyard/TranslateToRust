use std::io;
use std::cmp::{min, max};
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_substring_length(s: String, k: usize) -> bool {
        let n = s.len();
        let mut pos: HashMap<char, Vec<usize>> = HashMap::new();

        // Build the positional map for each character
        for (i, c) in s.chars().enumerate() {
            pos.entry(c).or_insert(Vec::new()).push(i);
        }

        let mut vec: Vec<(usize, usize)> = Vec::new();

        // Expand intervals based on character positions
        for (&c, positions) in pos.iter() {
            let mut l = positions[0];
            let mut r = *positions.last().unwrap();
            let mut flag = true;

            while flag {
                flag = false;

                // Check if other characters affect the current range
                for (_, other_positions) in pos.iter() {
                    let cnt = other_positions
                        .iter()
                        .filter(|&&x| x >= l && x <= r)
                        .count();

                    if cnt > 0 && cnt < other_positions.len() {
                        l = min(l, other_positions[0]);
                        r = max(r, *other_positions.last().unwrap());
                        flag = true;
                    }
                }
            }

            // Only consider intervals that do not fully cover the string
            if l > 0 || r < n - 1 {
                vec.push((r, l));
            }
        }

        // Sort intervals by ending position
        vec.sort_by(|a, b| a.0.cmp(&b.0));

        let mut r_boundary = usize::MAX;
        let mut cnt = 0;

        // Count non-overlapping regions
        for &(r, l) in &vec {
            if l > r_boundary {
                r_boundary = r;
                cnt += 1;
            }
        }

        cnt >= k
    }
}

fn main() {
    // Reading input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();

    let s = parts.next().unwrap().to_string();
    let k: usize = parts.next().unwrap().parse().unwrap();

    let solution = Solution;
    let result = solution.max_substring_length(s, k);
    
    // Output the result as a boolean (0 or 1)
    println!("{}", if result { 1 } else { 0 });
}