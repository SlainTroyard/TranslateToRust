use std::cmp::max;
use std::io::{self, BufRead};

fn max_possible_score(start: &mut [i32], d: i32) -> i32 {
    // Sort array
    start.sort();
    
    // Binary search
    let mut left = 0;
    let mut right = (start[start.len() - 1] + d - start[0]) / (start.len() as i32 - 1) + 1;
    
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        
        // Check function
        let mut sum = i64::MIN;
        let mut valid = true;
        for &val in start.iter() {
            sum = max(sum + mid as i64, val as i64);
            if sum > (val + d) as i64 {
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
    
    // Read first line containing startSize and d
    let line = lines.next().unwrap().unwrap();
    let mut values = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let start_size = values.next().unwrap();
    let d = values.next().unwrap();
    
    // Read the start array
    let line = lines.next().unwrap().unwrap();
    let mut start: Vec<i32> = line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    // Ensure we have the correct number of elements
    start.resize(start_size as usize, 0);
    
    // Calculate and print the result
    println!("{}", max_possible_score(&mut start, d));
}