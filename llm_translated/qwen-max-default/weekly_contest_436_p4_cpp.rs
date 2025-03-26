use std::io::{self, BufRead};

fn max_score(points: &Vec<i32>, m: i32) -> i64 {
    let check = |low: i64| -> bool {
        let n = points.len() as i32;
        let mut rem = m;
        let mut pre = 0;
        for (i, &point) in points.iter().enumerate() {
            let k = ((low - 1) / point as i64 + 1 - pre as i64) as i32;
            if i == n as usize - 1 && k <= 0 {
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
    };

    let mut left = 0;
    let mut right = (m + 1) / 2 * *points.iter().min().unwrap() as i64 + 1;
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line to get n and m
    let first_line = lines.next().unwrap().expect("Failed to read first line");
    let mut iter = first_line.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().expect("Failed to parse n");
    let m: i32 = iter.next().unwrap().parse().expect("Failed to parse m");

    // Parse the second line to get the points
    let second_line = lines.next().unwrap().expect("Failed to read second line");
    let points: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse point"))
        .collect();

    // Create a Solution instance and compute the result
    let result = max_score(&points, m);

    // Print the result to stdout
    println!("{}", result);
}