use std::io::{self, Read};
use std::cmp::{max, min};

fn max_possible_score(start: &mut [i32], start_size: usize, d: i32) -> i32 {
    // Sort array
    for i in 0..start_size - 1 {
        for j in 0..start_size - 1 - i {
            if start[j] > start[j + 1] {
                start.swap(j, j + 1);
            }
        }
    }
    
    // Binary search
    let mut left = 0;
    let mut right = (start[start_size - 1] + d - start[0]) / (start_size as i32 - 1) + 1;
    
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        
        // Check function
        let mut sum = i64::MIN;
        let mut valid = true;
        for i in 0..start_size {
            sum = max(sum + mid as i64, start[i] as i64);
            if sum > start[i] as i64 + d as i64 {
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

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let mut lines = input.lines();
    
    // Parse start_size and d
    let first_line: Vec<i32> = lines.next().unwrap().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let start_size = first_line[0] as usize;
    let d = first_line[1];
    
    // Parse start array
    let mut start: Vec<i32> = lines.next().unwrap().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and print result
    let result = max_possible_score(&mut start, start_size, d);
    println!("{}", result);
    
    Ok(())
}