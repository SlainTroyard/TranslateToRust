// Problem: Weekly Contest 437 Problem 3
use std::io;
use std::cmp;
use std::vec::Vec;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let s = parts[0].to_string();
    let k = parts[1].parse::<i32>().expect("Failed to parse K");
    let sol = Solution;
    println!("{}", sol.max_substring_length(s, k));
}

struct Solution;

impl Solution {
    pub fn max_substring_length(s: String, k: i32) -> bool {
        let n = s.len();
        if n == 0 || k == 0 {
            return true;
        }

        let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];
        for (i, c) in s.chars().enumerate() {
            let idx = c as usize - 'a' as usize;
            pos[idx].push(i);
        }

        let mut intervals: Vec<(usize, usize)> = Vec::new();

        for i in 0..26 {
            if pos[i].is_empty() {
                continue;
            }
            let mut l = pos[i][0];
            let mut r = *pos[i].last().unwrap();
            let mut expanded = true;

            while expanded {
                expanded = false;
                for j in 0..26 {
                    if j == i || pos[j].is_empty() {
                        continue;
                    }
                    let first = pos[j].binary_search(&l).unwrap_or_else(|x| x);
                    let last = pos[j].binary_search(&(r + 1)).unwrap_or_else(|x| x);
                    let cnt = last - first;
                    if cnt > 0 && cnt < pos[j].len() {
                        let new_l = cmp::min(l, pos[j][0]);
                        let new_r = cmp::max(r, *pos[j].last().unwrap());
                        if new_l != l || new_r != r {
                            l = new_l;
                            r = new_r;
                            expanded = true;
                        }
                    }
                }
            }

            if l > 0 || r < n - 1 {
                intervals.push((r, l));
            }
        }

        intervals.sort();

        let mut count = 0;
        let mut last_end = 0;
        for (r, l) in intervals {
            if l >= last_end {
                count += 1;
                last_end = r + 1;
            }
        }

        count >= k
    }
}