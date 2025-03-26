use std::io;
use std::str::FromStr;

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
            current_end = current_end.max(end);
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

fn is_poss(s: &str, n: i32, op: i32, mid: i32) -> bool {
    let mut i = 0;
    let mut j = 0;
    let mut zero = 0;
    let mut one = 0;
    let mut intervals = [[0; 2]; MAX_LEN];
    let mut size = 0;

    let s_bytes = s.as_bytes(); // Convert string to bytes for efficient indexing

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
                intervals[size][0] = i;
                intervals[size][1] = j;
                size += 1;
            }
        }
        j += 1;
    }

    let mut merged = [[0; 2]; MAX_LEN];
    let merged_size = merge(&intervals, size, mid, &mut merged);

    merged_size <= op as usize
}

fn get_mini(s: &str, n: i32, even_char: char, odd_char: char) -> i32 {
    let mut op = 0;
    for i in 0..n {
        if i % 2 == 0 && s.chars().nth(i as usize).unwrap() != even_char {
            op += 1;
        } else if i % 2 == 1 && s.chars().nth(i as usize).unwrap() != odd_char {
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
    let op_even_odd = get_mini(s, n, '0', '1');
    let op_odd_even = get_mini(s, n, '1', '0');
    op = op.min(op_even_odd);
    op = op.min(op_odd_even);

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
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim(); // Remove trailing newline

    let mut num_ops_str = String::new();
    io::stdin().read_line(&mut num_ops_str).expect("Failed to read line");
    let num_ops: i32 = num_ops_str.trim().parse().expect("Invalid input for numOps");

    let result = min_length(s, num_ops);
    println!("{}", result);
}