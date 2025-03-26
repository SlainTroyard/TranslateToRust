use std::io;

type Point = [i32; 2];

/// Calculates the area of a rectangle formed by four points if valid, otherwise returns -1.
///
/// # Arguments
///
/// * `points` - An array of four points forming potential rectangle.
///
/// # Returns
///
/// The area of the rectangle if valid, else -1.
fn calculate_area(points: &[[i32; 2]; 4]) -> i32 {
    let [p0, p1, p2, p3] = points;
    if p0[0] == p1[0] && p2[0] == p3[0] && p0[1] == p2[1] && p1[1] == p3[1] {
        let width = p2[0] - p0[0];
        let height = p1[1] - p0[1];
        width * height
    } else {
        -1
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    if n < 4 {
        println!("-1");
        return;
    }

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        points.push([x, y]);
    }

    // Sort points by x-coordinate, then by y-coordinate
    points.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

    let mut max_area = -1;
    for i in 0..(n - 3) {
        let mut j = i + 2;
        let y0 = points[i][1];
        let y1 = points[i + 1][1];

        // Find the first valid j where the y-coordinate is between y0 and y1
        while j < n - 2 {
            let current_y = points[j][1];
            if current_y > y1 || current_y < y0 {
                j += 1;
            } else {
                break;
            }
        }

        if j < n - 1 {
            let p0 = points[i];
            let p1 = points[i + 1];
            let p2 = points[j];
            let p3 = points[j + 1];
            let four_points = [p0, p1, p2, p3];
            let area = calculate_area(&four_points);
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("{}", max_area);
}