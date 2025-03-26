use std::io;

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
    let mut intervals = vec![];
    let mut size = 0;

    while j < n {
        if s.chars().nth(j).unwrap() == '0' {
            zero += 1;
        } else {
            one += 1;
        }

        while (j - i + 1) > mid {
            if s.chars().nth(i).unwrap() == '0' {
                zero -= 1;
            } else {
                one -= 1;
            }
            i += 1;
        }

        if (j - i + 1) == mid {
            if zero == 0 || one == 0 {
                intervals.push((i, j));
                size += 1;
            }
        }
        j += 1;
    }

    let mut merged = vec![(0, 0); MAX_LEN];
    let merged_size = merge(&intervals, mid as i32, &mut merged);

    merged_size <= op
}

fn get_mini(s: &str, n: usize, even: char, odd: char) -> usize {
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

const MAX_LEN: usize = 100_000;

fn min_length(s: &str, num_ops: usize) -> usize {
    let n = s.len();
    let mut start = 3;
    let mut end = n;
    let mut res = 2;

    let mut op = usize::MAX;
    let op_even_odd = get_mini(s, n, '0', '1');
    let op_odd_even = get_mini(s, n, '1', '0');
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
    let mut s = String::new();
    let mut num_ops = String::new();

    // Input the binary string
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s = s.trim().to_string();

    // Input the number of operations allowed
    io::stdin().read_line(&mut num_ops).expect("Failed to read line");
    let num_ops: usize = num_ops.trim().parse().expect("Please type a number!");

    // Compute the minimum length
    let result = min_length(&s, num_ops);

    // Output the result
    println!("{}", result);
}