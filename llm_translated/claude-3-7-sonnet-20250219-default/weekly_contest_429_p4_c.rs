use std::cmp::{max, min};
use std::io::{self, BufRead};

const MAX_LEN: usize = 100000;

/// Merges overlapping intervals when they are within the given length constraint
fn merge(intervals: &Vec<[usize; 2]>, size: usize, len: usize, merged: &mut Vec<[usize; 2]>) -> usize {
    if size == 0 {
        return 0;
    }

    let mut merged_size = 0;
    let mut current_start = intervals[0][0];
    let mut current_end = intervals[0][1];

    for i in 1..size {
        let start = intervals[i][0];
        let end = intervals[i][1];

        if start <= current_end && (start - current_start + 1) <= len {
            current_end = max(current_end, end);
        } else {
            merged[merged_size][0] = current_start;
            merged[merged_size][1] = current_end;
            merged_size += 1;
            current_start = start;
            current_end = end;
        }
    }
    
    merged[merged_size][0] = current_start;
    merged[merged_size][1] = current_end;
    merged_size + 1
}

/// Checks if it's possible to find a solution with at most 'op' operations for a given substring length 'mid'
fn is_poss(s: &str, n: usize, op: usize, mid: usize) -> bool {
    let mut i = 0;
    let mut j = 0;
    let mut zero = 0;
    let mut one = 0;
    let mut intervals = vec![[0; 2]; MAX_LEN];
    let mut size = 0;
    let chars: Vec<char> = s.chars().collect();

    while j < n {
        if chars[j] == '0' {
            zero += 1;
        } else {
            one += 1;
        }

        while j - i + 1 > mid {
            if chars[i] == '0' {
                zero -= 1;
            } else {
                one -= 1;
            }
            i += 1;
        }

        if j - i + 1 == mid {
            if zero == 0 || one == 0 {
                intervals[size][0] = i;
                intervals[size][1] = j;
                size += 1;
            }
        }
        j += 1;
    }

    let mut merged = vec![[0; 2]; MAX_LEN];
    let merged_size = merge(&intervals, size, mid, &mut merged);

    merged_size <= op
}

/// Calculates the minimum number of operations needed to make the string alternating
fn get_mini(s: &str, n: usize, even: char, odd: char) -> usize {
    let mut op = 0;
    let chars: Vec<char> = s.chars().collect();
    
    for i in 0..n {
        if i % 2 == 0 && chars[i] != even {
            op += 1;
        } else if i % 2 == 1 && chars[i] != odd {
            op += 1;
        }
    }
    op
}

/// Finds the minimum length of a string after at most 'num_ops' operations
fn min_length(s: &str, num_ops: usize) -> usize {
    let n = s.len();
    let mut start = 3;
    let mut end = n;
    let mut res = 2;

    let mut op = usize::MAX;
    let op_even_odd = get_mini(s, n, '0', '1');
    let op_odd_even = get_mini(s, n, '1', '0');
    op = min(op, op_even_odd);
    op = min(op, op_odd_even);

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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the binary string
    let s = lines.next().unwrap().unwrap();
    
    // Read the number of operations allowed
    let num_ops: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Compute the minimum length
    let result = min_length(&s, num_ops);
    
    // Output the result
    println!("{}", result);
}