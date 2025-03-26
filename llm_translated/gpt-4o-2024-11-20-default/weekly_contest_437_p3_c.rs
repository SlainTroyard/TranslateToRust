use std::io::{self, Write};
use std::cmp::{min, max};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Interval {
    left: usize,
    right: usize,
}

/// Binary search for lower_bound (find the first element greater than or equal to `val`)
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

/// Binary search for upper_bound (find the first element greater than `val`)
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

/// Main algorithm function
fn max_substring_length(s: &str, k: usize) -> bool {
    let n = s.len();
    let mut pos: Vec<VecDeque<usize>> = vec![VecDeque::new(); 26]; // Using VecDeque for efficient append operations

    // Populate positions for each character
    for (i, c) in s.chars().enumerate() {
        let idx = (c as u8 - b'a') as usize;
        pos[idx].push_back(i);
    }

    let mut intervals = Vec::new();

    for i in 0..26 {
        if !pos[i].is_empty() {
            let mut l = pos[i][0];
            let mut r = pos[i][pos[i].len() - 1];
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
                            r = max(r, pos[j][pos[j].len() - 1]);
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

    // Sort intervals by `right`
    intervals.sort_by(|a, b| a.right.cmp(&b.right));

    // Greedy selection of intervals
    let mut r = usize::MAX;
    let mut cnt = 0;

    for interval in intervals {
        if interval.left > r {