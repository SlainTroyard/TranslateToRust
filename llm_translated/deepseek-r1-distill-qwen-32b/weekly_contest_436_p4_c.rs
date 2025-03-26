// Weekly Contest 436 Problem 4
use std::io;

// Helper function to calculate the minimum element in an array
fn min_element(points: &[i32]) -> i32 {
    *points.iter().min().unwrap()
}

// Check function to determine if a certain score is achievable
fn check(points: &[i32], m: i32, low: i64) -> bool {
    let n = points.len();
    let mut rem = m;
    let mut pre = 0;
    for i in 0..n {
        let p = points[i] as i64;
        let k = ((low - 1) / p + 1 - pre) as i32;
        if i == n - 1 && k <= 0 {
            break;
        }
        let k = k.max(1);
        rem -= k * 2 - 1;
        if rem < 0 {
            return false;
        }
        pre = k - 1;
    }
    true
}

// Main algorithm function
fn max_score(points: &[i32], m: i32) -> i64 {
    let left = 0;
    let min_p = min_element(points) as i64;
    let right = ((m + 1) / 2 * min_p) + 1;

    let mut left = left;
    let mut right = right;

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

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    
    let n: i32 = iter.next().expect("Missing n").parse().expect("Invalid n");
    let m: i32 = iter.next().expect("Missing m").parse().expect("Invalid m");
    
    let mut points = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let val: i32 = iter.next().expect("Missing point value")
                            .parse()
                            .expect("Invalid point value");
        points.push(val);
    }
    
    // Compute result
    let result = max_score(&points, m);
    
    // Output result
    println!("{}", result);
}