use std::io::{self, BufRead};

// Converts a point (x, y) to a transformed value based on the side length.
fn zhuanhuan(point: &[i32; 2], side: i32) -> i64 {
    if point[0] >= point[1] {
        (point[0] + point[1]) as i64
    } else {
        let m = side as i64;
        4 * m - (point[0] + point[1]) as i64
    }
}

// Finds the next valid transformed value after adding 's' to the current value.
fn search(val: i64, r: &[i64], points_size: usize, side: i32) -> i64 {
    let m = side as i64;
    let val_mod = val % (4 * m);
    
    if val_mod > r[points_size - 1] {
        val - val_mod + 4 * m + r[0]
    } else if val_mod <= r[0] {
        val - val_mod + r[0]
    } else {
        let mut min = 0;
        let mut max = points_size - 1;
        
        // Binary search to find the insertion point for val_mod
        while min < max - 1 {
            let s = (min + max) / 2;
            if r[s] >= val_mod {
                max = s;
            } else {
                min = s;
            }
        }
        r[max] + (val - val_mod)
    }
}

// Checks if 's' is a valid maximum distance for the given configuration.
fn build(r: &[i64], s: i32, k: i32, side: i32, points_size: usize) -> bool {
    let s = s as i64;
    let m = side as i64;
    let k = k as i64;
    
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

// Calculates the maximum possible distance using binary search.
fn max_distance(side: i32, points: &Vec<[i32; 2]>, points_size: usize, k: i32) -> i32 {
    let mut r: Vec<i64> = points.iter()
        .map(|point| zhuanhuan(point, side))
        .collect();
    r.sort();
    
    let mut min = 1;
    let mut max = side + 1;
    
    // Binary search to find the maximum valid distance
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
    
    // Read first line containing side, number of points, and k
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let side: i32 = parts.next().unwrap().parse().unwrap();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    
    // Read each point's coordinates
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut coords = line.split_whitespace();
        let x: i32 = coords.next().unwrap().parse().unwrap();
        let y: i32 = coords.next().unwrap().parse().unwrap();
        points.push([x, y]);
    }
    
    // Compute and print the result
    let result = max_distance(side, &points, n, k);
    println!("{}", result);
}