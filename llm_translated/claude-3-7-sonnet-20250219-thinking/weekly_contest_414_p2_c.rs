use std::io;
use std::cmp;

fn max_possible_score(mut start: Vec<i32>, d: i32) -> i32 {
    let start_size = start.len();
    
    // Sort array using bubble sort to match original implementation
    for i in 0..start_size - 1 {
        for j in 0..start_size - i - 1 {
            if start[j] > start[j + 1] {
                let temp = start[j];
                start[j] = start[j + 1];
                start[j + 1] = temp;
            }
        }
    }
    
    // Binary search
    let mut left = 0;
    let mut right = (start[start_size - 1] + d - start[0])/(start_size as i32 - 1) + 1;
    
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        
        // Check function
        let mut sum = i64::MIN;
        let mut valid = true;
        for i in 0..start_size {
            sum = cmp::max(sum + mid as i64, start[i] as i64);
            if sum > (start[i] + d) as i64 {
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
    // Read start_size and d
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let parts: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    
    let start_size = parts[0] as usize;
    let d = parts[1];
    
    // Read array values
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let start: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    
    let result = max_possible_score(start, d);
    println!("{}", result);
}