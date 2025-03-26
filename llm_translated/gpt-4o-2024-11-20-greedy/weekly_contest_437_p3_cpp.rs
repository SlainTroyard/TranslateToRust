use std::io::{self, Read};
use std::cmp::{min, max};
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_substring_length(s: &str, k: usize) -> bool {
        let n = s.len();
        let mut pos: Vec<Vec<usize>> = vec![vec![]; 26];

        // Populate positions of each character
        for (i, c) in s.chars().enumerate() {
            let idx = (c as u8 - b'a') as usize;
            pos[idx].push(i);
        }

        let mut vec: Vec<(usize, usize)> = Vec::new();

        for i in 0..26 {
            if !pos[i].is_empty() {
                let mut l = pos[i][0];
                let mut r = *pos[i].last().unwrap();
                let mut flag = true;

                // Expand the range [l, r] to include other characters
                while flag {
                    flag = false;
                    for j in 0..26 {
                        if pos[j].is_empty() {
                            continue;
                        }
                        let cnt = pos[j]
                            .iter()
                            .filter(|&&x| x >= l && x <= r)
                            .count();
                        if cnt > 0 && cnt < pos[j].len() {
                            l = min(l, pos[j][0]);
                            r = max(r, *pos[j].last().unwrap());
                            flag = true;
                        }
                    }
                }

                // If the range doesn't cover the entire string, add it to the vector
                if l > 0 || r < n - 1 {
                    vec.push((r, l));
                }
            }
        }

        // Sort by the ending index, and count non-overlapping intervals
        vec.sort();
        let mut r = usize::MAX;
        let mut cnt = 0;

        for &(end, start) in &vec {
            if start > r {
                r = end;
                cnt += 1;
            }
        }

        cnt >= k
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // Parse the input
    let s = lines.next().unwrap();
    let k: usize = lines.next().unwrap().parse().unwrap();

    // Solve the problem
    let solution = Solution;
    let result = solution.max_substring_length(s, k);

    // Output the result
    println!("{}", result);
}