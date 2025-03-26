use std::cmp;
use std::io::{self, Write};

const MAX_LEN: usize = 100000;

fn merge(intervals: &[[i32; 2]], size: usize, len: i32, merged: &mut [[i32; 2]]) -> usize {
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
            current_end = cmp::max(current_end, end);
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

fn is_poss(s: &[u8], n: usize, op: i32, mid: i32) -> bool {
    let mut i = 0;
    let mut j = 0;
    let mut zero = 0;
    let mut one = 0;
    let mut intervals = vec![[0; 2]; MAX_LEN];
    let mut size = 0;

    while j < n {
        if s[j] == b'0' {
            zero += 1;
        } else {
            one += 1;
        }

        while (j - i + 1) as i32 > mid {
            if s[i] == b'0' {
                zero -= 1;
            } else {
                one -= 1;
            }
            i += 1;
        }

        if (j - i + 1) as i32 == mid {
            if zero == 0 || one == 0 {
                intervals[size][0] = i as i32;
                intervals[size][1] = j as i32;
                size += 1;
            }
        }
        j += 1;
    }

    let mut merged = vec![[0; 2]; MAX_LEN];
    let merged_size = merge(&intervals, size, mid, &mut merged);

    merged_size as i32 <= op
}

fn get_mini(s: &[u8], n: usize, even: u8, odd: u8) -> i32 {
    let mut op = 0;
    for i in 0..n {
        if i % 2 == 0 && s[i] != even {
            op += 1;
        } else if i % 2 == 1 && s[i] != odd {
            op += 1;
        }
    }
    op
}

fn min_length(s: &[u8], num_ops: i32) -> i32 {
    let n = s.len();
    let mut start = 3;
    let mut end = n as i32;
    let mut res = 2;

    let mut op = i32::MAX;
    let op_even_odd = get_mini(s, n, b'0', b'1');
    let op_odd_even = get_mini(s, n, b'1', b'0');
    op = cmp::min(op, op_even_odd);
    op = cmp::min(op, op_odd_even);

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
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim().as_bytes();

    let mut num_ops_str = String::new();
    io::stdin().read_line(&mut num_ops_str).expect("Failed to read line");
    let num_ops: i32 = num_ops_str.trim().parse().expect("Invalid number of operations");

    let result = min_length(s, num_ops);
    println!("{}", result);
}