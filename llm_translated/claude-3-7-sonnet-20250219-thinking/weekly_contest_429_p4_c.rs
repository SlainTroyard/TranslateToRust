use std::io::{self, BufRead};
use std::cmp;

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

fn is_poss(s: &str, n: usize, op: usize, mid: usize) -> bool {
    let s_bytes = s.as_bytes();
    let mut i = 0;
    let mut j = 0;
    let mut zero = 0;
    let mut one = 0;
    let mut intervals = [[0; 2]; MAX_LEN];
    let mut size = 0;

    while j < n {
        if s_bytes[j] == b'0' {
            zero += 1;
        } else {
            one += 1;
        }

        while (j - i + 1) > mid {
            if s_bytes[i] == b'0' {
                zero -= 1;
            } else {
                one -= 1;
            }
            i += 1;
        }

        if (j - i + 1) == mid {
            if zero == 0 || one == 0 {
                intervals[size][0] = i as i32;
                intervals[size][1] = j as i32;
                size += 1;
            }
        }
        j += 1;
    }

    let mut merged = [[0; 2]; MAX_LEN];
    let merged_size = merge(&intervals, size, mid as i32, &mut merged);

    merged_size <= op
}

fn get_mini(s: &str, n: usize, even: char, odd: char) -> usize {
    let s_bytes = s.as_bytes();
    let even_byte = even as u8;
    let odd_byte = odd as u8;
    let mut op = 0;

    for i in 0..n {
        if i % 2 == 0 && s_bytes[i] != even_byte {
            op += 1;
        } else if i % 2 == 1 && s_bytes[i] != odd_byte {
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
    let op_even_odd = get_mini(s, n, '0', '1');
    let op_odd_even = get_mini(s, n, '1', '0');
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input the binary string
    let s = lines.next().unwrap().unwrap();
    
    // Input the number of operations allowed
    let num_ops: usize = lines.next().unwrap().unwrap().parse().expect("Invalid number");
    
    // Compute the minimum length
    let result = min_length(&s, num_ops);
    
    // Output the result
    println!("{}", result);
}