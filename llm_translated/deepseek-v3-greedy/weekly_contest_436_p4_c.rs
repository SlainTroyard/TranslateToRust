use std::io::{self, Write};

// Returns the maximum of two integers
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// Returns the minimum element in a slice of integers
fn min_element(arr: &[i32]) -> i32 {
    let mut min_val = arr[0];
    for &num in arr.iter().skip(1) {
        if num < min_val {
            min_val = num;
        }
    }
    min_val
}

// Checks if it's possible to achieve the given score `low`
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

// Main function to calculate the maximum possible score
fn max_score(points: &[i32], m: i32) -> i64 {
    let mut left = 0;
    let right = ((m + 1) / 2) as i64 * min_element(points) as i64 + 1;
    
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

fn main() {
    // Read input for n and m
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input for n");
    let m: i32 = iter.next().unwrap().parse().expect("Invalid input for m");

    // Read the points array
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let point: i32 = input.trim().parse().expect("Invalid input for point");
        points.push(point);
    }

    // Calculate the result
    let result = max_score(&points, m);

    // Output the result
    println!("{}", result);
}