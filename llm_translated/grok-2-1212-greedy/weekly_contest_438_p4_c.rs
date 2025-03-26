use std::io::{self, BufRead};

fn zhuanhuan(point: &[i32; 2], side: i32) -> i64 {
    if point[0] >= point[1] {
        (point[0] + point[1]) as i64
    } else {
        let m = side as i64;
        4 * m - (point[0] + point[1]) as i64
    }
}

fn compar(a: &i64, b: &i64) -> std::cmp::Ordering {
    a.cmp(b)
}

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

fn build(r: &[i64], s: i32, k: i32, side: i32, points_size: usize) -> bool {
    let m = side as i64;
    for i in 0..points_size - 1 {
        let mut sum = 1;
        let mut th = r[i];
        let max_th = r[i] + 4 * m - s as i64;
        while th <= max_th {
            if sum == k as i64 {
                return true;
            }
            let val = th + s as i64;
            th = search(val, r, points_size, side);
            sum += 1;
        }
    }
    false
}

fn max_distance(side: i32, points: &[[i32; 2]], k: i32) -> i32 {
    let mut r: Vec<i64> = points.iter().map(|&point| zhuanhuan(&point, side)).collect();
    r.sort_by(compar);
    let mut min = 1;
    let mut max = side + 1;
    while min < max - 1 {
        let s = (min + max) / 2;
        if build(&r, s, k, side, r.len()) {
            min = s;
        } else {
            max = s;
        }
    }
    min
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read side, n, and k
    let first_line: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let side = first_line[0];
    let n = first_line[1];
    let k = first_line[2];

    // Read points
    let mut points = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let point: Vec<i32> = lines.next().unwrap()?.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        points.push([point[0], point[1]]);
    }

    // Calculate and print result
    let result = max_distance(side, &points, k);
    println!("{}", result);

    Ok(())
}