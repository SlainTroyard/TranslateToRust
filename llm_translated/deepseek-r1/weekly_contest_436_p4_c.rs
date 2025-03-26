use std::io::{self, BufRead};

fn check(points: &[i32], m: i32, low: i64) -> bool {
    let n = points.len();
    let mut rem = m;
    let mut pre = 0;

    for i in 0..n {
        let point = points[i] as i64;
        // Calculate k using the same formula as original code
        let k_i64 = (low - 1) / point + 1 - pre as i64;
        let mut k = k_i64 as i32;

        // Handle special case for last element where k could be <=0
        if i == n - 1 && k <= 0 {
            break;
        }

        // Ensure k is at least 1
        k = k.max(1);
        rem -= k * 2 - 1;

        if rem < 0 {
            return false;
        }

        pre = k - 1;
    }

    true
}

fn max_score(points: &[i32], m: i32) -> i64 {
    let min_val = *points.iter().min().unwrap() as i64;
    let m_i64 = m as i64;
    // Initialize binary search boundaries
    let mut left = 0;
    let mut right = ((m_i64 + 1) / 2) * min_val + 1;

    // Binary search for maximum achievable score
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
    let mut tokens = stdin
        .lock()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    // Read n and m from input
    let n = tokens.next().unwrap() as usize;
    let m = tokens.next().unwrap();

    // Read points array
    let points: Vec<i32> = (0..n).map(|_| tokens.next().unwrap()).collect();

    // Calculate and print result
    println!("{}", max_score(&points, m));
}