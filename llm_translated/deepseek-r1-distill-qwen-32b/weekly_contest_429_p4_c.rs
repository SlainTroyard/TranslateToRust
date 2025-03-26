use std::io;
use std::cmp;

fn merge(intervals: &Vec<(i32, i32)>, len: i32, merged: &mut Vec<(i32, i32)>) -> usize {
    if intervals.is_empty() {
        return 0;
    }

    let mut current_start = intervals[0].0;
    let mut current_end = intervals[0].1;
    let mut merged_size = 0;

    for i in 1..intervals.len() {
        let (start, end) = intervals[i];
        if start <= current_end && (start - current_start + 1) <= len {
            current_end = cmp::max(current_end, end);
        } else {
            merged.push((current_start, current_end));
            merged_size += 1;
            current_start = start;
            current_end = end;
        }
    }

    merged.push((current_start, current_end));
    merged_size + 1
}

fn is_possible(s: &str, n: usize, op: i32, mid: i32) -> bool {
    let mut i = 0;
    let mut j = 0;
    let mut zero = 0;
    let mut one = 0;
    let mut intervals = Vec::new();

    while j < n {
        if s.chars().nth(j).unwrap() == '0' {
            zero += 1;
        } else {
            one += 1;
        }

        while (j - i + 1) as i32 > mid {
            if s.chars().nth(i).unwrap() == '0' {
                zero -= 1;
            } else {
                one -= 1;
            }
            i += 1;
        }

        if (j - i + 1) as i32 == mid {
            if zero == 0 || one == 0 {
                intervals.push((i as i32, j as i32));
            }
        }
        j += 1;
    }

    let mut merged = Vec::new();
    let merged_size = merge(&intervals, mid, &mut merged);
    merged_size <= op as usize
}

fn get_min_operations(s: &str, n: usize, even_char: char, odd_char: char) -> i32 {
    let mut operations = 0;
    for i in 0..n {
        let c = s.chars().nth(i).unwrap();
        if i % 2 == 0 && c != even_char {
            operations += 1;
        } else if i % 2 == 1 && c != odd_char {
            operations += 1;
        }
    }
    operations
}

fn min_length(s: &str, num_ops: i32) -> i32 {
    let n = s.len();
    if n == 0 {
        return 0;
    }

    let op_even_odd = get_min_operations(s, n, '0', '1');
    let op_odd_even = get_min_operations(s, n, '1', '0');
    let min_op = cmp::min(op_even_odd, op_odd_even);

    if min_op <= num_ops {
        return 1;
    }

    let mut start = 3;
    let mut end = n as i32;
    let mut res = 2;

    while start <= end {
        let mid = start + (end - start) / 2;
        if is_possible(s, n, num_ops, mid) {
            end = mid - 1;
        } else {
            res = mid;
            start = mid + 1;
        }
    }

    res
}

fn main() -> io::Result<()> {
    let s = {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.trim().to_string()
    };

    let num_ops = {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.trim().parse().unwrap()
    };

    let result = min_length(&s, num_ops);
    println!("{}", result);
    Ok(())
}