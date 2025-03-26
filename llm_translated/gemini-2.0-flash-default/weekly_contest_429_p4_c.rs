use std::cmp::{min, max};
use std::io;

const MAX_LEN: usize = 100000;

fn merge(intervals: &[[i32; 2]], len: i32) -> Vec<[i32; 2]> {
    if intervals.is_empty() {
        return Vec::new();
    }

    let mut merged: Vec<[i32; 2]> = Vec::new();
    let mut current_start = intervals[0][0];
    let mut current_end = intervals[0][1];

    for i in 1..intervals.len() {
        let start = intervals[i][0];
        let end = intervals[i][1];

        if start <= current_end && (start - current_start + 1) <= len {
            current_end = max(current_end, end);
        } else {
            merged.push([current_start, current_end]);
            current_start = start;
            current_end = end;
        }
    }
    merged.push([current_start, current_end]);
    merged
}

fn is_poss(s: &str, op: i32, mid: i32) -> bool {
    let n = s.len() as i32;
    let mut i = 0;
    let mut j = 0;
    let mut zero = 0;
    let mut one = 0;
    let mut intervals: Vec<[i32; 2]> = Vec::new();

    let s_bytes = s.as_bytes();

    while j < n {
        if s_bytes[j as usize] == b'0' {
            zero += 1;
        } else {
            one += 1;
        }

        while (j - i + 1) > mid {
            if s_bytes[i as usize] == b'0' {
                zero -= 1;
            } else {
                one -= 1;
            }
            i += 1;
        }

        if (j - i + 1) == mid {
            if zero == 0 || one == 0 {
                intervals.push([i, j]);
            }
        }
        j += 1;
    }

    let merged = merge(&intervals, mid);

    merged.len() as i32 <= op
}

fn get_mini(s: &str, even: char, odd: char) -> i32 {
    let mut op = 0;
    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 && c != even {
            op += 1;
        } else if i % 2 == 1 && c != odd {
            op += 1;
        }
    }
    op
}

fn min_length(s: &str, num_ops: i32) -> i32 {
    let n = s.len() as i32;
    let mut start = 3;
    let mut end = n;
    let mut res = 2;

    let mut op = i32::MAX;
    let op_even_odd = get_mini(s, '0', '1');
    let op_odd_even = get_mini(s, '1', '0');
    op = min(op, op_even_odd);
    op = min(op, op_odd_even);

    if op <= num_ops {
        return 1;
    }

    while start <= end {
        let mid = start + (end - start) / 2;
        if is_poss(s, num_ops, mid) {
            end = mid - 1;
        } else {
            res = mid;
            start = mid + 1;
        }
    }
    res
}

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let s = s.trim();

    let mut num_ops_str = String::new();
    io::stdin().read_line(&mut num_ops_str)?;
    let num_ops: i32 = num_ops_str.trim().parse().unwrap();

    let result = min_length(s, num_ops);

    println!("{}", result);

    Ok(())
}