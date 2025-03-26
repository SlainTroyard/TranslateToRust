use std::io::{self, BufRead};

/// Returns the maximum of two integers
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

/// Returns the minimum value in the array
fn min_element(arr: &[i32]) -> i32 {
    let mut min_val = arr[0];
    for &val in arr.iter().skip(1) {
        if val < min_val {
            min_val = val;
        }
    }
    min_val
}

/// Check function - determines if it's possible to achieve the given score 'low'
fn check(points: &[i32], m: i32, low: i64) -> bool {
    let n = points.len();
    let mut rem = m;
    let mut pre = 0;
    
    for i in 0..n {
        let k = ((low - 1) / points[i] as i64 + 1 - pre as i64) as i32;
        if i == n - 1 && k <= 0 {
            break;
        }
        let k = max(k, 1);
        rem -= k * 2 - 1;
        if rem < 0 {
            return false;
        }
        pre = k - 1;
    }
    true
}

/// Main function implementation
fn max_score(points: &[i32], m: i32) -> i64 {
    let mut left: i64 = 0;
    // Calculate the right boundary for binary search
    let right: i64 = (m as i64 + 1) / 2 * min_element(points) as i64 + 1;
    
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