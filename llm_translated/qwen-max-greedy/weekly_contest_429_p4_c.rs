use std::io::{self, BufRead};

const MAX_LEN: usize = 100_000;

fn merge(intervals: &[(i32, i32)], len: i32, merged: &mut [(i32, i32)]) -> usize {
    if intervals.is_empty() {
        return 0;
    }

    let mut merged_size = 0;
    let mut current_start = intervals[0].0;
    let mut current_end = intervals[0].1;

    for (start, end) in intervals.iter().skip(1) {
        if *start <= current_end && (*start - current_start + 1) <= len {
            current_end = current_end.max(*end);
        } else {
            merged[merged_size] = (current_start, current_end);
            merged_size += 1;
            current_start = *start;
            current_end = *end;
        }
    }
    merged[merged_size] = (current_start, current_end);
    merged_size + 1
}

fn is_poss(s: &str, n: usize, op: usize, mid: usize) -> bool {
    let mut i = 0;
    let mut j = 0;
    let mut zero = 0;
    let mut one = 0;
    let mut intervals = [(0, 0); MAX_LEN];
    let mut size = 0;

    while j < n {
        if s.as_bytes()[j] == b'0' {
            zero += 1;
        } else {
            one += 1;
        }

        while (j - i + 1) > mid {
            if s.as_bytes()[i] == b'0' {
                zero -= 1;
            } else {
                one -= 1;
            }
            i += 1;
        }

        if (j - i + 1) == mid {
            if zero == 0 || one == 0 {
                intervals[size] = (i as i32, j as i32);
                size += 1;
            }
        }
        j += 1;
    }

    let mut merged = [(0, 0); MAX_LEN];
    let merged_size = merge(&intervals[..size], mid as i32, &mut merged);

    merged_size <= op
}

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

fn min_length(s: &str, num_ops: usize) -> usize {
    let n = s.len();
    let mut start = 3;
    let mut end = n;
    let mut res = 2;

    let mut op = usize::MAX;
    let op_even_odd = get_mini(s, n, b'0', b'1');
    let op_odd_even = get_mini(s, n, b'1', b'0');
    op = op.min(op_even_odd).min(op_odd_even);

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

    // Input the binary string
    let s = lines.next().unwrap().unwrap();

    // Input the number of operations allowed
    let num_ops: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Compute the minimum length
    let result = min_length(&s, num_ops);

    // Output the result
    println!("{}", result);
}