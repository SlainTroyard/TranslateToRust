use std::cmp::Ordering;
use std::io;

fn zhuanhuan(point: &[i32], side: i32) -> i64 {
    if point[0] >= point[1] {
        point[0] as i64 + point[1] as i64
    } else {
        let m = side as i64;
        4 * m - (point[0] as i64 + point[1] as i64)
    }
}

fn search(val: i64, r: &[i64], side: i32) -> i64 {
    let points_size = r.len();
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

fn build(r: &[i64], s: i32, k: i32, side: i32) -> bool {
    let points_size = r.len();
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
            th = search(val, r, side);
            sum += 1;
        }
    }
    false
}

fn max_distance(side: i32, points: &Vec<Vec<i32>>, k: i32) -> i32 {
    let points_size = points.len();
    let mut r = vec![0; points_size];
    for i in 0..points_size {
        r[i] = zhuanhuan(&points[i], side);
    }

    r.sort();

    let mut min = 1;
    let mut max = side + 1;
    while min < max - 1 {
        let s = (min + max) / 2;
        if build(&r, s, k, side) {
            min = s;
        } else {
            max = s;
        }
    }
    min
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.trim().split_whitespace();
    let side: i32 = iter.next().unwrap().parse().unwrap();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut points: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let mut iter = input.trim().split_whitespace();
        let point1: i32 = iter.next().unwrap().parse().unwrap();
        let point2: i32 = iter.next().unwrap().parse().unwrap();
        points.push(vec![point1, point2]);
    }
    
    println!("{}", max_distance(side, &points, k));
    Ok(())
}