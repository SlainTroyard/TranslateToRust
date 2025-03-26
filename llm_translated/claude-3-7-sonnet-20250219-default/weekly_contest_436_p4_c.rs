use std::io::{self, BufRead};

// Function to check if a given score can be achieved
fn check(points: &[i32], m: i32, low: i64) -> bool {
    let n = points.len();
    let mut rem = m;
    let mut pre = 0;
    
    for i in 0..n {
        let k = ((low - 1) / points[i] as i64 + 1 - pre as i64).max(1) as i32;
        
        if i == n - 1 && k <= 0 {
            break;
        }
        
        rem -= k * 2 - 1;
        if rem < 0 {
            return false;
        }
        pre = k - 1;
    }
    
    true
}

// Main function to calculate the maximum possible score
fn max_score(points: &[i32], m: i32) -> i64 {
    let mut left: i64 = 0;
    
    // Calculate the right boundary for binary search
    let min_val = *points.iter().min().unwrap() as i64;
    let right: i64 = (m as i64 + 1) / 2 * min_val + 1;
    
    // Binary search to find the maximum possible score
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(points, m, mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    
    left
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and m
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read points array
    let second_line = lines.next().unwrap()?;
    let points: Vec<i32> = second_line
        .split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and output the result
    let result = max_score(&points, m);
    println!("{}", result);
    
    Ok(())
}