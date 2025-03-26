use std::cmp::{max, min};
use std::io::{self, BufRead};
use std::process;

#[derive(Debug, Clone, Copy)]
struct Interval {
    left: i32,
    right: i32,
}

fn lower_bound(slice: &[i32], val: i32) -> usize {
    slice.partition_point(|&x| x < val)
}

fn upper_bound(slice: &[i32], val: i32) -> usize {
    slice.partition_point(|&x| x <= val)
}

fn max_substring_length(s: &str, k: i32) -> bool {
    let n = s.len() as i32;
    let mut pos = vec![vec![]; 26]; // Positions for each character 'a'-'z'

    // Populate character positions
    for (idx, ch) in s.chars().enumerate() {
        let c = (ch as u8 - b'a') as usize;
        pos[c].push(idx as i32);
    }

    let mut intervals = Vec::new();

    // Generate candidate intervals for each character
    for c in 0..26 {
        if !pos[c].is_empty() {
            let mut l = pos[c][0];
            let mut r = *pos[c].last().unwrap();
            let mut expanded;

            // Expand interval until no more changes
            loop {
                expanded = false;
                for other_c in 0..26 {
                    if pos[other_c].is_empty() {
                        continue;
                    }
                    let other_pos = &pos[other_c];
                    let low = lower_bound(other_pos, l);
                    let up = upper_bound(other_pos, r);
                    let cnt = up - low;

                    // Check if some but not all positions are contained
                    if cnt > 0 && cnt < other_pos.len() {
                        let new_l = min(l, other_pos[0]);
                        let new_r = max(r, *other_pos.last().unwrap());
                        if new_l < l || new_r > r {
                            l = new_l;
                            r = new_r;
                            expanded = true;
                        }
                    }
                }
                if !expanded {
                    break;
                }
            }

            // Add interval if it doesn't cover the entire string
            if l > 0 || r < n - 1 {
                intervals.push(Interval { left: l, right: r });
            }
        }
    }

    // Sort intervals by right endpoint for greedy selection
    intervals.sort_by_key(|interval| interval.right);

    // Select maximum non-overlapping intervals
    let mut current_end = -1;
    let mut count = 0;
    for interval in intervals {
        if interval.left > current_end {
            current_end = interval.right;
            count += 1;
        }
    }

    count >= k
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 2 {
        eprintln!("Error reading input");
        process::exit(1);
    }

    let s = parts[0];
    let k: i32 = parts[1].parse().unwrap_or_else(|_| {
        eprintln!("Invalid k value");
        process::exit(1);
    });

    let result = max_substring_length(s, k);
    println!("{}", if result { "true" } else { "false" });
}