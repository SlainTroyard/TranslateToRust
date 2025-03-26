// Problem: Weekly Contest 414 Problem 2
use std::io;

fn max_possible_score(start: &mut [i32], d: i32) -> i32 {
    // Sort the array in ascending order
    start.sort();

    let n = start.len();
    if n == 0 {
        return 0;
    }

    // Calculate initial right value
    let first = start[0] as i64;
    let last = start[n - 1] as i64;
    let mut right = ((last + d as i64 - first) / (n as i64 - 1)) + 1;
    let mut left = 0;

    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if is_valid(start, mid, d) {
            left = mid;
        } else {
            right = mid;
        }
    }

    left
}

fn is_valid(start: &[i32], mid: i64, d: i32) -> bool {
    let mut sum = i64::MIN;
    for &num in start {
        sum = sum.max((num as i64) + mid);
        if sum > (num as i64) + d as i64 {
            return false;
        }
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut parts = input.trim().split_whitespace();
    let start_size = parts.next().and_then(|s| s.parse().ok()).expect("Invalid start size");
    let d = parts.next().and_then(|s| s.parse().ok()).expect("Invalid d value");

    let mut start = Vec::with_capacity(start_size);
    for _ in 0..start_size {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("Failed to read number");
        let num = num_str.trim().parse().expect("Invalid number");
        start.push(num);
    }

    let result = max_possible_score(&mut start, d);
    println!("{}", result);
}