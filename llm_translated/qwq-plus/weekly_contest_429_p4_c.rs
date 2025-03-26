use std::io;

fn get_mini(s: &str, even_char: char, odd_char: char) -> i32 {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 && c != even_char {
                1
            } else if i % 2 == 1 && c != odd_char {
                1
            } else {
                0
            }
        })
        .sum()
}

fn merge(intervals: &[[i32; 2]], len: i32) -> Vec<[i32; 2]> {
    if intervals.is_empty() {
        return Vec::new();
    }

    let mut merged = Vec::new();
    let mut current_start = intervals[0][0];
    let mut current_end = intervals[0][1];

    for &(start, end) in intervals.iter().skip(1) {
        if start <= current_end && (start - current_start + 1) <= len {
            if end > current_end {
                current_end = end;
            }
        } else {
            merged.push([current_start, current_end]);
            current_start = start;
            current_end = end;
        }
    }
    merged.push([current_start, current_end]);
    merged
}

fn is_possible(s: &str, op: i32, mid: i32) -> bool {
    let n = s.len();
    let s = s.as_bytes();
    let mid = mid as usize;
    let mut i = 0;
    let mut j = 0;
    let mut zero = 0;
    let mut one = 0;
    let mut intervals = Vec::new();

    while j < n {
        let c = s[j];
        if c == b'0' {
            zero += 1;
        } else {
            one += 1;
        }

        while (j - i + 1) > mid {
            let c_remove = s[i];
            if c_remove == b'0' {
                zero -= 1;
            } else {
                one -= 1;
            }
            i += 1;
        }

        if (j - i + 1) == mid {
            if zero == 0 || one == 0 {
                intervals.push([i as i32, j as i32]);
            }
        }
        j += 1;
    }

    let merged = merge(&intervals, mid as i32);
    merged.len() as i32 <= op
}

fn min_length(s: &str, num_ops: i32) -> i32 {
    let n = s.len() as i32;
    if n == 0 {
        return 0;
    }

    let mut res = 2;
    let mut op = i32::MAX;
    let op_even_odd = get_mini(s, '0', '1');
    let op_odd_even = get_mini(s, '1', '0');
    op = op.min(op_even_odd);
    op = op.min(op_odd_even);

    if op <= num_ops {
        return 1;
    }

    let mut start = 3;
    let mut end = n;
    while start <= end {
        let mid = start + (end - start) / 2;
        if is_possible(s, num_ops, mid) {
            end = mid - 1;
            res = mid;
        } else {
            start = mid + 1;
        }
    }
    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");
    let mut parts = input.split_whitespace();
    let s = parts.next().expect("No string provided");
    let num_ops: i32 = parts.next().expect("No number provided")
        .parse().expect("Invalid number");

    let result = min_length(s, num_ops);
    println!("{}", result);
}