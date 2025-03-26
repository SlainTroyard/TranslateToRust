use std::cmp::{max, min};
use std::io;

/// Merges overlapping intervals where the segment length is constrained by `len`.
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

/// Determines if it's possible to divide the string `s` into `op` operations with length `mid`.
fn is_poss(s: &str, n: usize, op: usize, mid: usize) -> bool {
    let mut i = 0;
    let mut j = 0;
    let mut zero = 0;
    let mut one = 0;

    let mut intervals = vec![];

    while j < n {
        if s.as_bytes()[j] == b'0' {
            zero += 1;
        } else {
            one += 1;
        }

        while j - i + 1 > mid {
            if s.as_bytes()[i] == b'0' {
                zero -= 1;
            } else {
                one -= 1;
            }
            i += 1;
        }

        if j - i + 1 == mid && (zero == 0 || one == 0) {
            intervals.push((i, j));
        }
        
        j += 1;
    }

    let merged = merge(&intervals, mid);
    merged.len() <= op
}

/// Computes the minimum number of operations required to make the string alternate with `even` and `odd`.
fn get_mini(s: &str, n: usize, even: u8, odd: u8) -> usize {
    let mut op = 0;
    for (i, &ch) in s.as_bytes().iter().enumerate() {
        if i % 2 == 0 && ch != even {
            op += 1;
        } else if i % 2 == 1 && ch != odd {
            op += 1;
        }
    }
    op
}

/// Finds the minimum length possible for the string after `num_ops` operations.
fn min_length(s: &str, num_ops: usize) -> usize {
    let n = s.len();
    let mut start = 3;
    let mut end = n;
    let mut res = 2;

    let op_even_odd = get_mini(s, n, b'0', b'1');
    let op_odd_even = get_mini(s, n, b'1', b'0');
    let mut op = usize::min(op_even_odd, op_odd_even);

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

fn main() {
    let mut input = String::new();

    // Read the binary string
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim();
    input.clear();

    // Read the number of operations allowed
    io::stdin().read_line(&mut input).unwrap();
    let num_ops: usize = input.trim().parse().unwrap();

    // Compute the minimum length
    let result = min_length(s, num_ops);

    // Output the result
    println!("{}", result);
}