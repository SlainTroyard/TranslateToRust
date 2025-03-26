use std::io::{self, BufRead};

fn zhuanhuan(point: &[i32], side: i32) -> i64 {
    if point[0] >= point[1] {
        (point[0] + point[1]) as i64
    } else {
        let m = side as i64;
        4 * m - (point[0] + point[1]) as i64
    }
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

fn build(r: &[i64], s: i32, k: i32, side: i32, points_size: usize) -> bool {
    let s = s as i64;
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

fn max_distance(side: i32, points: &[Vec<i32>], points_size: usize, _points_col_size: &i32, k: i32) -> i32 {
    let mut r = vec![0i64; points_size];
    
    for i in 0..points_size {
        r[i] = zhuanhuan(&points[i], side);
    }
    
    r.sort();
    
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line containing side, n, k
    let first_line = lines.next().unwrap().unwrap();
    let params: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let side = params[0];
    let n = params[1] as usize;
    let k = params[2];
    
    // Read the points
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let point: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        points.push(point);
    }
    
    let points_col_size = 2;
    println!("{}", max_distance(side, &points, n, &points_col_size, k));
}