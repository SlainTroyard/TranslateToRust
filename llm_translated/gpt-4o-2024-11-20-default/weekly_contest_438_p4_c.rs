use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::vec::Vec;

fn zhuanhuan(point: &[i32], side: i32) -> i64 {
    if point[0] >= point[1] {
        (point[0] + point[1]) as i64
    } else {
        let m = side as i64;
        4 * m - ((point[0] + point[1]) as i64)
    }
}

fn compar(a: &i64, b: &i64) -> Ordering {
    a.cmp(b)
}

fn search(val: i64, r: &[i64], points_size: usize, side: i32) -> i64 {
    let m = side as i64;
    let val1 = val % (4 * m);

    if val1 > r[points_size - 1] {
        return val - val1 + 4 * m + r[0];
    }
    if val1 <= r[0] {
        return val - val1 + r[0];
    }

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

fn build(r: &[i64], s: i64, k: i32, side: i32, points_size: usize) -> bool {
    let m = side as i64;
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

fn max_distance(side: i32, points: Vec<[i32; 2]>, k: i32) -> i32 {
    let mut r: Vec<i64> = points.iter().map(|point| zhuanhuan(point, side)).collect();
    r.sort_by(|a, b| compar(a, b));

    let (mut min, mut max) = (1, side + 1);
    while min < max - 1 {
        let s = (min + max) / 2;
        if build(&r, s as i64, k, side, points.len()) {
            min = s;
        } else {
            max = s;
        }
    }
    min
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_split = first_line.split_whitespace();
    let side: i32 = first_line_split.next().unwrap().parse().unwrap();
    let n: usize = first_line_split.next().unwrap().parse().unwrap();
    let k: i32 = first_line_split.next().unwrap().parse().unwrap();

    // Parse the points
    let mut points: Vec<[i32; 2]> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut line_split = line.split_whitespace();
        let x: i32 = line_split.next().unwrap().parse().unwrap();
        let y: i32 = line_split.next().unwrap().parse().unwrap();
        points.push([x, y]);
    }

    // Calculate the maximum distance and print the result
    let result = max_distance(side, points, k);
    println!("{}", result);
}