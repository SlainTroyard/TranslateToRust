use std::cmp::Ordering;
use std::io;

// Function to convert a point to a single value based on the side length
fn zhuanhuan(point: &[i32], side: i32) -> i64 {
    if point[0] >= point[1] {
        (point[0] + point[1]) as i64
    } else {
        let m = side as i64;
        4 * m - (point[0] + point[1]) as i64
    }
}

// Comparator function for sorting
fn compar(a: &i64, b: &i64) -> Ordering {
    a.cmp(b)
}

// Function to search for the next value in the sorted array
fn search(val: i64, r: &[i64], points_size: usize, side: i32) -> i64 {
    let m = side as i64;
    let val1 = val % (4 * m);
    if val1 > r[points_size - 1] {
        val - val1 + 4 * m + r[0]
    } else if val1 <= r[0] {
        val - val1 + r[0]
    } else {
        let mut min = 0;
        let mut max = points_size - 1;
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

// Function to check if it's possible to build a solution with given parameters
fn build(r: &[i64], s: i32, k: i32, side: i32, points_size: usize) -> bool {
    let m = side as i64;
    for i in 0..points_size - 1 {
        let mut sum = 1;
        let mut th = r[i];
        let max_th = r[i] + 4 * m - s as i64;
        while th <= max_th {
            if sum == k {
                return true;
            }
            let val = th + s as i64;
            th = search(val, r, points_size, side);
            sum += 1;
        }
    }
    false
}

// Main function to find the maximum distance
fn max_distance(side: i32, points: &[Vec<i32>], points_size: usize, k: i32) -> i32 {
    let mut r = vec![0; points_size];
    for i in 0..points_size {
        r[i] = zhuanhuan(&points[i], side);
    }
    r.sort_by(compar);
    let mut min = 1;
    let mut max = side + 1;
    while min < max - 1 {
        let s = (min + max) / 2;
        if build(&r, s, k, side, points_size) {
            min = s;
        } else {
            max = s;
        }
    }
    min
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let side: i32 = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        points.push(vec![x, y]);
    }

    let points_col_size = 2;
    println!("{}", max_distance(side, &points, n, k));
}