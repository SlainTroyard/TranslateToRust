use std::io::{self, BufRead};

/// Performs a specialized transformation of a point, mirroring the C function "zhuanhuan".
/// If point[0] >= point[1], returns point[0] + point[1].
/// Otherwise, returns 4 * side - (point[0] + point[1]).
fn zhuanhuan(point: &[i64; 2], side: i64) -> i64 {
    if point[0] >= point[1] {
        point[0] + point[1]
    } else {
        4 * side - (point[0] + point[1])
    }
}

/// Finds the next transformed value in the sequence given the current value "val",
/// based on the sorted array "r" of transformed points. This mirrors the C function "search".
fn search(val: i64, r: &[i64], side: i64) -> i64 {
    let m = side;
    let val1 = val % (4 * m);

    // If val1 is larger than the largest element, wrap around
    if val1 > r[r.len() - 1] {
        return val - val1 + 4 * m + r[0];
    }
    // If val1 is smaller than or equal to the smallest element, go to the smallest element
    if val1 <= r[0] {
        return val - val1 + r[0];
    }

    // Binary search to find the smallest r[s] >= val1
    let mut min = 0;
    let mut max = r.len() - 1;
    while min < max - 1 {
        let mid = (min + max) / 2;
        if r[mid] >= val1 {
            max = mid;
        } else {
            min = mid;
        }
    }

    // Offset the found value by (val - val1) to keep the progression
    r[max] + (val - val1)
}

/// Attempts to build a sequence of length k using step s, checking if it is possible
/// to place k points on the perimeter with at least s distance apart. Mirrors the C function "build".
fn build(r: &[i64], s: i64, k: i64, side: i64) -> bool {
    let m = side;
    // We only iterate up to pointsSize-1 because we move from r[i] forward in the while loop
    for i in 0..(r.len() - 1) {
        let mut sum = 1;          // We've placed the first point
        let mut th = r[i];        // Current threshold
        let max_th = r[i] + 4 * m - s; // We won't exceed this threshold in the sequence

        while th <= max_th {
            // If we've placed k points already, we succeed
            if sum == k {
                return true;
            }
            // Compute next threshold
            let val = th + s;
            th = search(val, r, side);
            sum += 1;
        }
    }
    false
}

/// Determines the maximum distance s such that k points can be placed
/// on the perimeter of the square [0,side] x [0,side] with at least distance s apart.
/// Mirrors the C function "maxDistance".
fn max_distance(side: i64, points: &[[i64; 2]], k: i64) -> i64 {
    // Transform points
    let mut r: Vec<i64> = points.iter().map(|p| zhuanhuan(p, side)).collect();
    // Sort transformed points
    r.sort_unstable();

    // Binary search for the maximum distance
    let mut min_s = 1;
    let mut max_s = side + 1;

    while min_s < max_s - 1 {
        let s = (min_s + max_s) / 2;
        if build(&r, s, k, side) {
            min_s = s;
        } else {
            max_s = s;
        }
    }

    min_s
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read side, n, and k (simulates scanf("%d %d %d", &side, &n, &k))
    let first_line = lines
        .next()
        .expect("Failed to read input for side, n, k")
        .expect("Could not parse line");
    let mut split = first_line.split_whitespace();
    let side: i64 = split.next().expect("Missing side").parse().expect("Invalid side");
    let n: i64 = split.next().expect("Missing n").parse().expect("Invalid n");
    let k: i64 = split.next().expect("Missing k").parse().expect("Invalid k");

    // Read n lines of points
    let mut points = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let line = lines
            .next()
            .expect("Failed to read input for points")
            .expect("Could not parse line");
        let mut coords = line.split_whitespace();
        let x: i64 = coords.next().expect("Missing x").parse().expect("Invalid x");
        let y: i64 = coords.next().expect("Missing y").parse().expect("Invalid y");
        points.push([x, y]);
    }

    // Compute and print the result (simulates printf("%d\n", maxDistance(...)))
    let result = max_distance(side, &points, k);
    println!("{}", result);
}