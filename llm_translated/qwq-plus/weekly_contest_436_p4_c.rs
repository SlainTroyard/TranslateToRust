use std::io;

// Returns the minimum value in the array
fn min_element(points: &[i32]) -> i32 {
    *points.iter().min().unwrap()
}

// Check function to determine if a given score is achievable
fn check(points: &[i32], m: i32, low: i64) -> bool {
    let n = points.len();
    let mut rem = m;
    let mut pre = 0;

    for (i, &point) in points.iter().enumerate() {
        let point_val = point as i64;
        let k = ((low - 1) / point_val + 1) - pre;

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

// Main function to compute the maximum score using binary search
fn max_score(points: &[i32], m: i32) -> i64 {
    let mut left: i64 = 0;
    let min_val = min_element(points) as i64;
    // Calculate the right boundary for binary search
    let right = ((m as i64 + 1) / 2 * min_val) + 1;

    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(points, m, mid) {
            left = mid;
        } else {
            right -= (right - mid);
        }
    }

    left
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Invalid integer"));

    let n = tokens.next().expect("Need n");
    let m = tokens.next().expect("Need m");

    let points: Vec<i32> = tokens.take(n as usize).collect();
    if points.len() != n as usize {
        eprintln!("Not enough points provided");
        std::process::exit(1);
    }

    let result = max_score(&points, m);
    println!("{}", result);
}