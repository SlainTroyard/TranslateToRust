use std::io::{self, Write};
use std::cmp::{min, max};

// Helper function to perform binary search for lower_bound
fn lower_bound(arr: &[usize], val: usize) -> usize {
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

// Helper function to perform binary search for upper_bound
fn upper_bound(arr: &[usize], val: usize) -> usize {
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

// Struct to represent an interval
#[derive(Debug, Clone, Copy)]
struct Interval {
    left: usize,
    right: usize,
}

// Main function to solve the problem
fn max_substring_length(s: &str, k: usize) -> bool {
    let n = s.len();
    let s_bytes = s.as_bytes();

    // Store positions of each character
    let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];

    for (i, &c) in s_bytes.iter().enumerate() {
        pos[(c - b'a') as usize].push(i);
    }

    // Store candidate intervals
    let mut intervals = Vec::new();

    // Find valid intervals
    for i in 0..26 {
        if !pos[i].is_empty() {
            let mut l = pos[i][0];
            let mut r = *pos[i].last().unwrap();
            let mut flag = true;

            while flag {
                flag = false;
                for j in 0..26 {
                    if !pos[j].is_empty() {
                        let low_idx = lower_bound(&pos[j], l);
                        let up_idx = upper_bound(&pos[j], r);
                        let cnt = up_idx - low_idx;

                        if cnt > 0 && cnt < pos[j].len() {
                            l = min(l, pos[j][0]);
                            r = max(r, *pos[j].last().unwrap());
                            flag = true;
                        }
                    }
                }
            }

            if l > 0 || r < n - 1 {
                intervals.push(Interval { left: l, right: r });
            }
        }
    }

    // Sort intervals by right endpoint
    intervals.sort_by_key(|interval| interval.right);

    // Greedily select intervals
    let mut r = usize::MAX;
    let mut cnt = 0;
    for interval in intervals {
        if interval.left > r {
            r = interval.right;
            cnt += 1;
        }
    }

    cnt >= k
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let mut parts = input.split_whitespace();
    let s = parts.next().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Call the function and print the result
    let result = max_substring_length(s, k);
    println!("{}", if result { "true" } else { "false" });
}