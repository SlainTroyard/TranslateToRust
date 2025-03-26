use std::cmp::{max, min};
use std::io::{self, Write};

fn main() {
    // Read the binary string from stdin
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read input");
    let s = s.trim(); // Remove any trailing newline

    // Read the number of operations from stdin
    let mut num_ops_str = String::new();
    io::stdin().read_line(&mut num_ops_str).expect("Failed to read input");
    let num_ops: i32 = num_ops_str.trim().parse().expect("Invalid number of operations");

    // Compute the result
    let result = min_length(s, num_ops);

    // Output the result
    println!("{}", result);
}

fn merge(intervals: Vec<Vec<i32>>, len: i32) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return vec![];
    }
    let n = intervals.len();

    let mut res = vec![];
    res.push(intervals[0].clone());

    for i in 1..n {
        let curr_start = intervals[i][0];
        let curr_end = intervals[i][1];

        if curr_start <= res.last().unwrap()[1] && (curr_start - res.last().unwrap()[0] + 1) <= len {
            let last_end = res.last_mut().unwrap()[1];
            *last_end = max(last_end, curr_end);
        } else {
            res.push(intervals[i].clone());
        }
    }
    res
}

fn is_poss(s: &str, op: i32, mid: i32) -> bool {
    let n = s.len();
    let mut i = 0;
    let mut j = 0;
    let mut zero = 0;
    let mut one = 0;
    let mut intervals = vec![];

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
                intervals.push(vec![i as i32, j as i32]);
            }
        }
        j += 1;
    }

    let merged = merge(intervals, mid);
    merged.len() as i32 <= op
}

fn get_mini(s: &str, even: char, odd: char) -> i32 {
    let n = s.len();
    let mut op = 0;
    for (i, c) in s.chars().enumerate() {
        if i % 2 == 1 && c != odd {
            op += 1;
        } else if i % 2 == 0 && c != even {
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

    let mut op = n as i32 + 1;
    op = min(op, get_mini(s, '0', '1'));
    op = min(op, get_mini(s, '1', '0'));
    if op <= num_ops {
        return 1;
    }

    while start <= end {
        let mid = start + (end - start) / 2;
        let curr = is_poss(s, num_ops, mid);
        if curr {
            end = mid - 1;
        } else {
            res = mid;
            start = mid + 1;
        }
    }
    res
}