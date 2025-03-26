use std::cmp;
use std::io::{self, BufRead};

fn max_possible_score(start: &mut [i32], d: i32) -> i32 {
    // Sort the array using bubble sort
    let n = start.len();
    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if start[j] > start[j + 1] {
                start.swap(j, j + 1);
            }
        }
    }

    // Binary search to find the maximum possible score
    let mut left = 0;
    let mut right = (start[n - 1] + d - start[0]) / (n as i32 - 1) + 1;

    while left + 1 < right {
        let mid = left + (right - left) / 2;

        // Check if the current mid is a valid score
        let mut sum = i64::MIN;
        let mut valid = true;
        for &s in start.iter() {
            sum = cmp::max(sum + mid as i64, s as i64);
            if sum > s as i64 + d as i64 {
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
    
    // Read the first line for startSize and d
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let start_size: usize = parts.next().unwrap().parse().unwrap();
    let d: i32 = parts.next().unwrap().parse().unwrap();

    // Read the second line for the start array
    let second_line = lines.next().unwrap().unwrap();
    let mut start: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Compute and print the result
    println!("{}", max_possible_score(&mut start, d));
}