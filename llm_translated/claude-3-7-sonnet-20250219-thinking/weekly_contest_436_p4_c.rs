use std::io::{self, BufRead};
use std::cmp;

// Returns the minimum value in the array
fn min_element(arr: &[i32]) -> i32 {
    let mut min_val = arr[0];
    for &val in &arr[1..] {
        if val < min_val {
            min_val = val;
        }
    }
    min_val
}

// Check function - determines if a given score 'low' is achievable
fn check(points: &[i32], m: i32, low: i64) -> bool {
    let n = points.len();
    let mut rem = m;
    let mut pre = 0;
    
    for i in 0..n {
        let k = ((low - 1) / points[i] as i64 + 1 - pre as i64) as i32;
        
        if i == n - 1 && k <= 0 {
            break;
        }
        
        let k = cmp::max(k, 1);
        rem -= k * 2 - 1;
        
        if rem < 0 {
            return false;
        }
        
        pre = k - 1;
    }
    
    true
}

// Main algorithm implementation
fn max_score(points: &[i32], m: i32) -> i64 {
    let mut left = 0;
    // Calculate right boundary for binary search
    let mut right = (m + 1) as i64 / 2 * min_element(points) as i64 + 1;
    
    // Binary search for the maximum possible score
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
    let line = lines.next().unwrap()?;
    let mut parts = line.split_whitespace();
    
    let n: usize = parts.next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Error reading input for n"))?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid n"))?;
    
    let m: i32 = parts.next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Error reading input for m"))?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid m"))?;
    
    // Allocate memory for points
    let mut points = Vec::with_capacity(n);
    
    // Read points array
    for i in 0..n {
        let line = lines.next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, format!("Error reading input for points[{}]", i)))??;
        
        let point: i32 = line.trim().parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, format!("Invalid point at index {}", i)))?;
        
        points.push(point);
    }
    
    // Calculate result
    let result = max_score(&points, m);
    
    // Output result
    println!("{}", result);
    
    Ok(())
}