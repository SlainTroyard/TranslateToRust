use std::cmp::{max, min};
use std::io::{self, Write};

// Helper function to merge intervals
fn merge(intervals: &[(usize, usize)], len: usize) -> Vec<(usize, usize)> {
    if intervals.is_empty() {
        return vec![];
    }

    let mut merged = Vec::new();
    let mut current_start = intervals[0].0;
    let mut current_end = intervals[0].1;

    for &(start, end) in &intervals[1..] {
        if start <= current_end && (start - current_start + 1) <= len {
            current_end = max(current_end, end);
        } else {
            merged.push((current_start, current_end));
            current_start = start;
            current_end = end;
        }
    }
    merged.push((current_start, current_end));
    merged
}

// Helper function to check if it is possible with the given parameters
fn is_poss(s: &[u8], n: usize, op: usize, mid: usize) -> bool {
    let mut i = 0;
    let mut zero_count = 0;
    let mut one_count = 0;
    let mut intervals = Vec::new();

    for j in 0..n {
        if s[j] == b'0' {
            zero_count += 1;
        } else {
            one_count += 1;
        }

        while (j - i + 1) > mid {
            if s[i] == b'0' {
                zero_count -= 1;
            } else {
                one_count -= 1;
            }
            i += 1;
        }

        if (j - i + 1) == mid {
            if zero_count == 0 || one_count == 0 {
                intervals.push((i, j));
            }
        }
    }

    let merged = merge(&intervals, mid);
    merged.len() <= op
}

// Helper function to calculate minimum operations for a specific pattern
fn get_mini(s: &[u8], n: usize, even: u8, odd: u8) -> usize {
    let mut op = 0;
    for i in 0..n {
        if (i % 2 == 0 && s[i] != even) || (i % 2 == 1 && s[i] != odd) {
            op += 1;
        }
    }
    op
}

// Main function to calculate the minimum length with the given number of operations
fn min_length(s: &str, num_ops: usize) -> usize {
    let n = s.len();
    let s = s.as_bytes();
    let mut start = 3;
    let mut end = n;
    let mut res = 2;

    let op_even_odd = get_mini(s, n, b'0', b'1');
    let op_odd_even = get_mini(s, n, b'1', b'0');
    let mut op = min(op_even_odd, op_odd_even);

    if op <= num_ops {
        return 1;
    }

    while start <= end {
        let mid = start + (end - start) / 2;
        if is_poss(s, n, num_ops, mid) {
            end = mid - 1;
        } else {
            res = mid;
            start = mid + 1;
        }
    }
    res
}

// Main function with input/output handling
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim().to_string();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_ops: usize = input.trim().parse().expect("Failed to parse number");

    let result = min_length(&s, num_ops);

    println!("{}", result);
}