use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input into a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    // Split the input by whitespace to get tokens.
    let mut tokens = input.split_whitespace();

    // Read n and m from the input.
    let n: usize = tokens
        .next()
        .ok_or("Expected a value for n")?
        .parse()
        .map_err(|_| "Failed to parse n")?;
    let m: i64 = tokens
        .next()
        .ok_or("Expected a value for m")?
        .parse()
        .map_err(|_| "Failed to parse m")?;

    // Read the points vector. There are n integers.
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let point: i64 = tokens
            .next()
            .ok_or("Expected a point value")?
            .parse()
            .map_err(|_| "Failed to parse point")?;
        points.push(point);
    }

    // Compute and print the maximum score.
    println!("{}", max_score(&points, m));

    Ok(())
}

/// Computes the maximum score based on the given points and m.
/// This function implements the same logic as the original C++ code.
fn max_score(points: &[i64], m: i64) -> i64 {
    // check is a closure that, given a score "low", verifies if it satisfies
    // the problem's conditions.
    let check = |low: i64| -> bool {
        let n = points.len();
        let mut rem = m;
        let mut pre = 0;
        for (i, &point) in points.iter().enumerate() {
            // Calculate k using ceiling division equivalent:
            // k = ceil(low / point) - pre which is computed as:
            // k = ((low - 1) / point) + 1 - pre.
            let mut k = (low - 1) / point + 1 - pre;
            // Special case for the last element: if k <= 0, then break.
            if i == n - 1 && k <= 0 {
                break;
            }
            // Ensure that k is at least 1.
            if k < 1 {
                k = 1;
            }
            // Deduct the required moves from rem.
            rem -= k * 2 - 1;
            if rem < 0 {
                return false;
            }
            pre = k - 1;
        }
        true
    };

    // Initialize binary search bounds.
    // left is 0 and right is computed as:
    // ((m + 1) / 2) * (minimum point in points) + 1.
    let min_point = *points
        .iter()
        .min()
        .expect("There must be at least one element in points");
    let mut left: i64 = 0;
    let mut right: i64 = ((m + 1) / 2) * min_point + 1;

    // Perform binary search to find the maximum valid score.
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