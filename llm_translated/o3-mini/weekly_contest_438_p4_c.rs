use std::error::Error;
use std::io::{self, BufRead};

// Given a point (two integers), convert it into its "arc-length" representation on the perimeter.
// If the first coordinate is greater than or equal to the second, the value is the sum of the two;
// otherwise, it's 4*m - (the sum), where m is the side length.
fn zhuanhuan(point: &[i32], side: i32) -> i64 {
    let m = side as i64;
    if point[0] >= point[1] {
        point[0] as i64 + point[1] as i64
    } else {
        4 * m - (point[0] as i64 + point[1] as i64)
    }
}

// Given a value 'val' and a sorted slice 'r' (representing the converted points along the perimeter),
// this function finds the next arc-length value in the circle (which is cyclical).
fn search(val: i64, r: &[i64], side: i32) -> i64 {
    let m = side as i64;
    let modulo = 4 * m;
    // Get the remainder value in the interval [0, 4*m)
    let val1 = val % modulo;
    let len = r.len();
    // Case when the remainder is larger than the last element in the sorted list:
    if val1 > r[len - 1] {
        return val - val1 + modulo + r[0];
    }
    // Case when the remainder is not greater than the first element:
    if val1 <= r[0] {
        return val - val1 + r[0];
    }
    // Binary search for the smallest element in r that is not less than val1.
    let mut lo = 0;
    let mut hi = len - 1;
    while lo < hi - 1 {
        let mid = (lo + hi) / 2;
        if r[mid] >= val1 {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    r[hi] + val - val1
}

// For a given step size 's', this function checks if it's possible to build a chain
// of exactly 'k' points such that the difference (arc distance) between consecutive points is at least s.
fn build(r: &[i64], s: i64, k: i32, side: i32) -> bool {
    let m = side as i64;
    let len = r.len();
    // Iterate over each starting point (except the last one)
    for i in 0..(len - 1) {
        let mut count = 1;
        let mut current_threshold = r[i];
        // Define the maximum threshold allowed for the starting point so that the total arc length does not exceed a full lap.
        let max_threshold = r[i] + 4 * m - s;
        // Continue while the threshold is within allowable arc length.
        while current_threshold <= max_threshold {
            if count == k {
                return true;
            }
            let next_val = current_threshold + s;
            current_threshold = search(next_val, r, side);
            count += 1;
        }
    }
    false
}

// This function applies binary search over possible step sizes (s) and returns the maximum distance
// that satisfies the condition specified (build returns true).
fn max_distance(side: i32, points: &[ [i32; 2] ], k: i32) -> i32 {
    let points_size = points.len();
    // Convert each point into its arc-length representation.
    let mut r: Vec<i64> = points.iter().map(|pt| zhuanhuan(pt, side)).collect();
    // Sort the converted points.
    r.sort_unstable();

    // Binary search boundaries: s ranges from 1 to side (inclusive lower bound, exclusive upper bound s = side+1)
    let (mut low, mut high) = (1, side + 1);
    while low < high - 1 {
        let s = (low + high) / 2;
        if build(&r, s as i64, k, side) {
            low = s;
        } else {
            high = s;
        }
    }
    low
}

fn main() -> Result<(), Box<dyn Error>> {
    // Use a buffered reader for efficient input handling.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line containing "side n k"
    let first_line = lines.next().ok_or("No input provided")??;
    let mut nums = first_line.split_whitespace();
    let side: i32 = nums.next().ok_or("Missing side")?.parse()?;
    let n: usize = nums.next().ok_or("Missing n")?.parse()?;
    let k: i32 = nums.next().ok_or("Missing k")?.parse()?;

    // Read the next n lines, each containing two integers (the coordinates)
    let mut points: Vec<[i32; 2]> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().ok_or("Not enough points")??;
        let mut iter = line.split_whitespace();
        let a: i32 = iter.next().ok_or("Missing point coordinate")?.parse()?;
        let b: i32 = iter.next().ok_or("Missing point coordinate")?.parse()?;
        points.push([a, b]);
    }

    // Call the max_distance function and print the result.
    let ans = max_distance(side, &points, k);
    println!("{}", ans);

    Ok(())
}