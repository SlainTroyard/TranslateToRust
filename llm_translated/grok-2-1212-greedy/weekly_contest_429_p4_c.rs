use std::io::{self, Read};

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
                intervals[size][0] = i as i32;
                intervals[size][1] = j as i32;
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
    for i in 0..n {
        if i % 2 == 0 && s.chars().nth(i).unwrap() != even {
            op += 1;
        } else if i % 2 == 1 && s.chars().nth(i).unwrap() != odd {
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let s = lines.next().unwrap().trim();
    let num_ops: i32 = lines.next().unwrap().trim().parse().unwrap();

    let result = min_length(s, num_ops);

    println!("{}", result);

    Ok(())
}