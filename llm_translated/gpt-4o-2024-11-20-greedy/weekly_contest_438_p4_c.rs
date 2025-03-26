use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::mem;

// Converts a point to its corresponding value on the "path".
fn zhuanhuan(point: &[i32], side: i64) -> i64 {
    let m = side;
    if point[0] >= point[1] {
        (point[0] + point[1]) as i64
    } else {
        4 * m - (point[0] + point[1]) as i64
    }
}

// Custom binary search function.
fn search(val: i64, r: &[i64], points_size: usize, side: i64) -> i64 {
    let m = side;
    let val1 = val % (4 * m);
    if val1 > r[points_size - 1] {
        val - val1 + 4 * m + r[0]
    } else if val1 <= r[0] {
        val - val1 + r[0]
    } else {
        let (mut min, mut max) = (0, points_size - 1);
        while min < max - 1 {
            let s = (max + min) / 2;
            if r[s] >= val1 {
                max = s;
            } else {
                min = s;
            }
        }
        r[max] + val - val1
    }
}

// Checks if it's possible to build k islands given a distance threshold.
fn build(r: &[i64], s: i64, k: i32, side: i64, points_size: usize) -> bool {
    let m = side;
    for i in 0..points_size - 1 {
        let mut sum = 1;
        let mut th = r[i];
        let max_th = r[i] + 4 * m - s;
        while th <= max_th {
            if sum == k {
                return true;
            }
            let val = th + s;
            th = search(val, r, points_size, side);
            sum += 1;
        }
    }
    false
}

// Determines the maximum possible distance that can satisfy the constraints.
fn max_distance(side: i64, points: Vec<[i32; 2]>, k: i32) -> i32 {
    let points_size = points.len();
    let mut r: Vec<i64> = points.iter().map(|point| zhuanhuan(point, side)).collect();
    r.sort_unstable();

    let (mut min, mut max) = (1, side + 1);
    while min < max - 1 {
        let s = (min + max) / 2;
        if build(&r, s, k, side, points_size) {
            min = s;
        } else {
            max = s;
        }
    }
    min as i32
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line which contains side, number of points, and k.
    let first_line = lines.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let side: i64 = first_iter.next().unwrap().parse().unwrap();
    let n: usize = first_iter.next().unwrap().parse().unwrap();
    let k: i32 = first_iter.next().unwrap().parse().unwrap();

    // Parse the following n lines for the points.
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(line) = lines.next() {
            let coords: Vec<i32> = line
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            points.push([coords[0], coords[1]]);
        }
    }

    // Calculate the result and print it.
    let result = max_distance(side, points, k);
    println!("{}", result);
}