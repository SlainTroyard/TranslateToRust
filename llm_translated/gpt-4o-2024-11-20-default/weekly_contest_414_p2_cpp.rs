use std::cmp::max;
use std::io::{self, BufRead};

// Helper function to check if a given score is valid
fn check(start: &[i32], d: i32, score: i32) -> bool {
    let mut x: i64 = i64::MIN;

    for &s in start {
        x = max(x + score as i64, s as i64);
        if x > s as i64 + d as i64 {
            return false;
        }
    }

    true
}

// Main solution function
fn max_possible_score(start: &mut Vec<i32>, d: i32) -> i32 {
    start.sort_unstable();

    let mut left = 0;
    let mut right = (start.last().unwrap() + d - start[0]) / (start.len() as i32 - 1) + 1;

    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(start, d, mid) {
            left = mid;
        } else {
            right = mid;
        }
    }

    left
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line: startSize and d
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let start_size: usize = iter.next().unwrap().parse().unwrap();
    let d: i32 = iter.next().unwrap().parse().unwrap();

    // Second line: elements of the start vector
    let second_line = lines.next().unwrap().unwrap();
    let start: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    assert_eq!(start.len(), start_size, "Mismatched input size.");

    // Solve the problem
    let mut start_mut = start.clone(); // explicitly clone for mutation
    let result = max_possible_score(&mut start_mut, d);

    // Output the result
    println!("{}", result);
}