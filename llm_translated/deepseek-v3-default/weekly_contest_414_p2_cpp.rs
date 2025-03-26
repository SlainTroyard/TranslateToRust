use std::cmp::max;
use std::io::{self, BufRead};

fn max_possible_score(start: &mut Vec<i32>, d: i32) -> i32 {
    start.sort();

    let check = |score: i32| -> bool {
        let mut x = i64::MIN;
        for &s in start.iter() {
            x = max(x + score as i64, s as i64);
            if x > s as i64 + d as i64 {
                return false;
            }
        }
        true
    };

    let mut left = 0;
    let mut right = (start.last().unwrap() + d - start[0]) / (start.len() as i32 - 1) + 1;

    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(mid) {
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

    // Read the first line: startSize and d
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let start_size: usize = parts.next().unwrap().parse().unwrap();
    let d: i32 = parts.next().unwrap().parse().unwrap();

    // Read the second line: the start vector
    let second_line = lines.next().unwrap().unwrap();
    let mut start: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Ensure the start vector has the correct size
    assert_eq!(start.len(), start_size);

    // Compute and print the result
    let result = max_possible_score(&mut start, d);
    println!("{}", result);
}