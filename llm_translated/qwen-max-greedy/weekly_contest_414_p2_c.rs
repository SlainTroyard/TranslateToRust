use std::io::{self, BufRead, Write};

fn max_possible_score(start: &mut [i32], d: i32) -> i32 {
    // Sort array
    start.sort_unstable();

    // Binary search
    let mut left = 0;
    let mut right = (start[start.len() - 1] + d - start[0]) / (start.len() as i32 - 1) + 1;

    while left + 1 < right {
        let mid = left + (right - left) / 2;

        // Check function
        let mut sum = i64::MIN;
        let mut valid = true;
        for &value in start.iter() {
            sum = sum.max(0) + mid as i64;
            if sum > value as i64 + d as i64 {
                valid = false;
                break;
            }
        }

        if valid {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();
    stdin_lock.read_line(&mut buffer).expect("Failed to read line");
    let mut iter = buffer.split_whitespace();
    let start_size: usize = iter.next().unwrap().parse().expect("Invalid input");
    let d: i32 = iter.next().unwrap().parse().expect("Invalid input");

    let mut start = vec![0; start_size];
    for i in 0..start_size {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).expect("Failed to read line");
        start[i] = buffer.trim().parse().expect("Invalid input");
    }

    // Compute and print result
    let result = max_possible_score(&mut start, d);
    println!("{}", result);
}