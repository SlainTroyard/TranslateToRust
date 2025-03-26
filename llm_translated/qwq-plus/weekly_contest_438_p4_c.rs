use std::io;

fn zhuanhuan(point: &[i32], side: i32) -> i64 {
    let x = point[0];
    let y = point[1];
    if x >= y {
        (x + y) as i64
    } else {
        let m = side as i64;
        4 * m - (x + y) as i64
    }
}

fn search(val: i64, r: &[i64], side: i32) -> i64 {
    let m = side as i64;
    let four_m = 4 * m;
    let val1 = val % four_m;
    let len = r.len();
    if val1 > r[len - 1] {
        return val - val1 + four_m + r[0];
    }
    if val1 <= r[0] {
        return val - val1 + r[0];
    }
    let idx = r.binary_search(&val1).unwrap_or_else(|x| x);
    r[idx] + (val - val1)
}

fn build(r: &[i64], s: i64, k: i32, side: i32) -> bool {
    let m = side as i64;
    let four_m = 4 * m;
    for &start in &r[..r.len() - 1] {
        let mut sum = 1;
        let mut th = start;
        let max_th = start + (four_m - s);
        while th <= max_th {
            if sum == k as i64 {
                return true;
            }
            let val = th + s;
            th = search(val, r, side);
            sum += 1;
        }
    }
    false
}

fn max_distance(side: i32, points: &[[i32; 2]], k: i32) -> i32 {
    let points_size = points.len();
    let mut r = Vec::with_capacity(points_size);
    for p in points {
        r.push(zhuanhuan(p, side));
    }
    r.sort_unstable();
    let mut min = 1;
    let mut max = side + 1;
    while min < max - 1 {
        let s = (min + max) / 2;
        if build(&r, s as i64, k, side) {
            min = s;
        } else {
            max = s;
        }
    }
    min
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let side: i32 = parts.next().unwrap().parse().unwrap();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut nums = line.split_whitespace();
        let x: i32 = nums.next().unwrap().parse().unwrap();
        let y: i32 = nums.next().unwrap().parse().unwrap();
        points.push([x, y]);
    }

    let result = max_distance(side, &points, k);
    println!("{}", result);
}