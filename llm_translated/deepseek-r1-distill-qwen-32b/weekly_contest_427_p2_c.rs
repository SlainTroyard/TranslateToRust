use std::io;

fn get(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32), p4: (i32, i32)) -> i32 {
    if p1.0 == p2.0 && p3.0 == p4.0 && p1.1 == p3.1 && p2.1 == p4.1 {
        let width = (p3.0 - p1.0).abs();
        let height = (p2.1 - p1.1).abs();
        width * height
    } else {
        -1
    }
}

fn max_rectangle_area(points: &mut Vec<(i32, i32)>) -> i32 {
    let n = points.len() as i32;
    let mut max_area = -1;

    for i in 0..(n as usize - 3) {
        let p1 = points[i];
        let p2 = points[i + 1];
        let mut j = i + 2;

        while j < (n as usize - 2) {
            if points[j].1 > p2.1 || points[j].1 < p1.1 {
                j += 1;
            } else {
                break;
            }
        }

        if j <= (n as usize - 2) {
            let p3 = points[j];
            let p4 = points[j + 1];
            let area = get(p1, p2, p3, p4);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read n");
    let n: i32 = input.trim().parse().expect("Invalid n");

    if n < 4 {
        println!("-1");
        return;
    }

    let mut points = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read point");
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        let x = parts[0].parse().expect("Invalid x");
        let y = parts[1].parse().expect("Invalid y");
        points.push((x, y));
    }

    points.sort();

    let result = max_rectangle_area(&mut points);
    println!("{}", result);
}