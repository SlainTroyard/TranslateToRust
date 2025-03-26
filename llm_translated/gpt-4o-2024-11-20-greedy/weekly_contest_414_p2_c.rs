use std::io::{self, BufRead};
use std::cmp::min;

fn max_possible_score(start: &mut Vec<i32>, d: i32) -> i32 {
    // Sort the array
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
            sum = min(sum + mid as i64, value as i64);
            if sum > (value + d) as i64 {
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read startSize and d
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let start_size: usize = first_line_iter.next().unwrap().parse().unwrap();
    let d: i32 = first_line_iter.next().unwrap().parse().unwrap();

    // Read the start array
    let second_line = lines.next().unwrap().unwrap();
    let mut start: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    assert_eq!(start.len(), start_size);

    // Compute and print the result
    let result = max_possible_score(&mut start, d);
    println!("{}", result);
}