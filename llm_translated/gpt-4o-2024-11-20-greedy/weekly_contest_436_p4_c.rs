use std::io;
use std::cmp::min;

// Returns the maximum of two numbers
fn max(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

// Returns the minimum element in an array
fn min_element(arr: &[i64]) -> i64 {
    let mut min_val = arr[0];
    for &val in arr.iter().skip(1) {
        if val < min_val {
            min_val = val;
        }
    }
    min_val
}

// Check function - determines if the score `low` can be achieved
fn check(points: &[i64], m: i64, low: i64) -> bool {
    let n = points.len();
    let mut rem = m;
    let mut pre = 0;

    for (i, &point) in points.iter().enumerate() {
        let mut k = (low - 1) / point + 1 - pre;
        if i == n - 1 && k <= 0 {
            break;
        }
        k = max(k, 1);
        rem -= k * 2 - 1;
        if rem < 0 {
            return false;
        }
        pre = k - 1;
    }
    true
}

// Main function to calculate the maximum score
fn max_score(points: &[i64], m: i64) -> i64 {
    let mut left = 0;
    let right = (m + 1) / 2 * min_element(points) + 1;

    // Binary search for the maximum possible score
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
    let stdin = io::stdin();
    let mut buffer = String::new();

    // Read n (number of points) and m
    stdin.read_line(&mut buffer).unwrap();
    let mut iter = buffer.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: i64 = iter.next().unwrap().parse().unwrap();

    // Read the points array
    buffer.clear();
    stdin.read_line(&mut buffer).unwrap();
    let points: Vec<i64> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    if points.len() != n {
        eprintln!("Error: Expected {} points, but got {}", n, points.len());
        return;
    }

    // Compute the result using max_score function
    let result = max_score(&points, m);

    // Print the result to stdout
    println!("{}", result);
}