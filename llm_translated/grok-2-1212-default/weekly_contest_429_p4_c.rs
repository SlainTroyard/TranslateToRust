use std::io::{self, BufRead};

const MAX_LEN: usize = 100_000;

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
            merged[merged_size] = [current_start, current_end];
            merged_size += 1;
            current_start = start;
            current_end = end;
        }
    }
    merged[merged_size] = [current_start, current_end];
    merged_size + 1
}

fn is_poss(s: &str, n: usize, op: i32, mid: i32) -> bool {
    let mut i = 0;
    let mut j = 0;
    let mut zero = 0;
    let mut one = 0;
    let mut intervals = [[0; 2]; MAX_LEN];
    let mut size = 0;

    while j < n {
        if s.chars().nth(j).unwrap() == '0' {
            zero += 1;
        } else {
            one += 1;
        }

        while (j as i32 - i as i32 + 1) > mid {
            if s.chars().nth(i).unwrap() == '0' {
                zero -= 1;
            } else {
                one -= 1;
            }
            i += 1;
        }

        if (j as i32 - i as i32 + 1) == mid {
            if zero == 0 || one == 0 {
                intervals[size] = [i as i32, j as i32];
                size += 1;
            }
        }
        j += 1;
    }

    let mut merged = [[0; 2]; MAX_LEN];
    let merged_size = merge(&intervals[0..size], size, mid, &mut merged);

    merged_size as i32 <= op
}

fn get_mini(s: &str, n: usize, even: char, odd: char) -> i32 {
    let mut op = 0;
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if i % 2 == 0 && c != even as u8 {
            op += 1;
        } else if i % 2 == 1 && c != odd as u8 {
            op += 1;
        }
    }
    op
}

fn min_length(s: &str, num_ops: i32) -> i32 {
    let n = s.len();
    let mut start = 3;
    let mut end = n as i32;
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input the binary string
    let s = lines.next().unwrap()?.trim().to_string();

    // Input the number of operations allowed
    let num_ops: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // Compute the minimum length
    let result = min_length(&s, num_ops);

    // Output the result
    println!("{}", result);

    Ok(())
}