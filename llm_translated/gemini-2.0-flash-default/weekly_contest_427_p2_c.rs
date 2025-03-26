use std::cmp::Ordering;
use std::io;

const MAX_POINTS: usize = 100;

// Helper function to calculate the maximum rectangle area given four points
fn get(points: &[[i32; 2]; 4]) -> i32 {
    let max_area = if points[0][0] == points[1][0]
        && points[2][0] == points[3][0]
        && points[0][1] == points[2][1]
        && points[1][1] == points[3][1]
    {
        let area = (points[2][0] - points[0][0]) * (points[1][1] - points[0][1]);
        area
    } else {
        -1
    };
    max_area
}

// Comparison function for sorting points by x-coordinate, then y-coordinate
fn compare_points(a: &[i32; 2], b: &[i32; 2]) -> Ordering {
    if a[0] != b[0] {
        a[0].cmp(&b[0])
    } else {
        a[1].cmp(&b[1])
    }
}

fn max_rectangle_area(points: &mut [[i32; 2]], n: usize) -> i32 {
    let mut max_area = -1;

    // Sort the points by their x-coordinates and then by y-coordinates
    points[..n].sort_by(compare_points);

    // Iterate over the points to find all potential rectangles
    for i in 0..n.saturating_sub(3) {
        let mut rectangle_points = [[0i32; 2]; 4];

        // Add the first two points of the rectangle
        rectangle_points[0][0] = points[i][0];
        rectangle_points[0][1] = points[i][1];
        rectangle_points[1][0] = points[i + 1][0];
        rectangle_points[1][1] = points[i + 1][1];

        // Find the next two points that complete the rectangle
        let mut j = i + 2;
        while j < n.saturating_sub(1) {
            if points[j][1] > points[i + 1][1] || points[j][1] < points[i][1] {
                j += 1;
            } else {
                break;
            }
        }

        // Add the potential third and fourth points
        if j < n.saturating_sub(1) {
            rectangle_points[2][0] = points[j][0];
            rectangle_points[2][1] = points[j][1];
            rectangle_points[3][0] = points[j + 1][0];
            rectangle_points[3][1] = points[j + 1][1];

            // Calculate the maximum area for this set of points
            let area = get(&rectangle_points);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn main() -> io::Result<()> {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str)?;
    let n: usize = n_str.trim().parse().unwrap();

    if n < 4 {
        println!("-1");
        return Ok(());
    }

    let mut points = [[0i32; 2]; MAX_POINTS];
    for i in 0..n {
        let mut point_str = String::new();
        io::stdin().read_line(&mut point_str)?;
        let mut iter = point_str.trim().split_whitespace();
        points[i][0] = iter.next().unwrap().parse().unwrap();
        points[i][1] = iter.next().unwrap().parse().unwrap();
    }

    let result = max_rectangle_area(&mut points, n);
    println!("{}", result);

    Ok(())
}