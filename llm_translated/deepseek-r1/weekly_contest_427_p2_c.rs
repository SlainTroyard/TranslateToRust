use std::cmp::Ordering;
use std::io;

fn calculate_max_area(points: &[[i32; 2]; 4]) -> i32 {
    if points[0][0] == points[1][0]
        && points[2][0] == points[3][0]
        && points[0][1] == points[2][1]
        && points[1][1] == points[3][1]
    {
        (points[2][0] - points[0][0]) * (points[1][1] - points[0][1])
    } else {
        -1
    }
}

fn max_rectangle_area(points: &[(i32, i32)]) -> i32 {
    let n = points.len();
    let mut max_area = -1;

    for i in 0..n.saturating_sub(3) {
        let (p0, p1) = (points[i], points[i + 1]);
        let mut j = i + 2;

        while j < n.saturating_sub(2) && (points[j].1 > p1.1 || points[j].1 < p0.1) {
            j += 1;
        }

        if j < n.saturating_sub(1) {
            let rect = [
                [p0.0, p0.1],
                [p1.0, p1.1],
                [points[j].0, points[j].1],
                [points[j + 1].0, points[j + 1].1],
            ];
            max_area = max_area.max(calculate_max_area(&rect));
        }
    }

    max_area
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap_or(0);

    if n < 4 {
        println!("-1");
        return;
    }

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut parts = line.split_whitespace();
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        points.push((x, y));
    }

    points.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    println!("{}", max_rectangle_area(&points));
}