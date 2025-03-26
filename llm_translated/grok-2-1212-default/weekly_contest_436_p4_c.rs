use std::io::{self, BufRead};

// Returns the larger of two numbers
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// Returns the minimum value in the array
fn min_element(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

// Check function - determines if it's possible to achieve the given score 'low'
fn check(points: &[i32], m: i32, low: i64) -> bool {
    let n = points.len();
    let mut rem = m;
    let mut pre = 0;

    for (i, &point) in points.iter().enumerate() {
        let k = ((low - 1) / point as i64 + 1 - pre as i64) as i32;
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

// Main function implementation
fn max_score(points: &[i32], m: i32) -> i64 {
    let mut left = 0;
    // Calculate the right boundary for binary search
    let right = (m as i64 + 1) / 2 * min_element(points) as i64 + 1;

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
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap()?;
    let mut numbers = first_line.split_whitespace();
    let n: usize = numbers.next().unwrap().parse().unwrap();
    let m: i32 = numbers.next().unwrap().parse().unwrap();

    // Read points array
    let points_line = lines.next().unwrap()?;
    let points: Vec<i32> = points_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call function to calculate result
    let result = max_score(&points, m);

    // Output result
    println!("{}", result);

    Ok(())
}