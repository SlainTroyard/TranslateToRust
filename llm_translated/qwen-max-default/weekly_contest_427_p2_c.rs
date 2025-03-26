use std::cmp::Ordering;
use std::io::{self, BufRead};

const MAX_POINTS: usize = 100;

// Helper function to calculate the maximum rectangle area given four points
fn get(points: &[[i32; 2]; 4]) -> i32 {
    let mut max_area = -1;

    // Check if these points can form a rectangle:
    // - First and second points have the same x-coordinate
    // - Third and fourth points have the same x-coordinate
    // - First and third points have the same y-coordinate
    // - Second and fourth points have the same y-coordinate
    if points[0][0] == points[1][0] &&
       points[2][0] == points[3][0] &&
       points[0][1] == points[2][1] &&
       points[1][1] == points[3][1] {
        // Calculate the area of the rectangle
        let area = (points[2][0] - points[0][0]).abs() * 
                   (points[1][1] - points[0][1]).abs();
        if area > max_area {
            max_area = area;
        }
    }

    max_area
}

// Comparison function for sorting points by x-coordinate, then y-coordinate
fn compare_points(a: &&[i32; 2], b: &&[i32; 2]) -> Ordering {
    if a[0] != b[0] {
        a[0].cmp(&b[0])
    } else {
        a[1].cmp(&b[1])
    }
}

fn max_rectangle_area(points: &mut [[i32; 2]; MAX_POINTS], n: usize) -> i32 {
    let mut max_area = -1;

    // Sort the points by their x-coordinates and then by y-coordinates
    points[..n].sort_unstable_by(compare_points);

    // Iterate over the points to find all potential rectangles
    for i in 0..n-3 {
        let mut rectangle_points = [[0; 2]; 4];

        // Add the first two points of the rectangle
        rectangle_points[0] = points[i];
        rectangle_points[1] = points[i + 1];

        // Find the next two points that complete the rectangle
        let mut j = i + 2;
        while j < n - 1 {
            if points[j][1] > points[i + 1][1] || points[j][1] < points[i][1] {
                j += 1;
            } else {
                break;
            }
        }

        // Add the potential third and fourth points
        if j < n - 1 {
            rectangle_points[2] = points[j];
            rectangle_points[3] = points[j + 1];

            // Calculate the maximum area for this set of points
            let area = get(&rectangle_points);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of points
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    if n < 4 {
        println!("-1");
        return;
    }

    let mut points: [[i32; 2]; MAX_POINTS] = [[0; 2]; MAX_POINTS];

    // Read the points
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace()
                                   .map(|x| x.parse().unwrap())
                                   .collect();
        points[i] = [coords[0], coords[1]];
    }

    // Calculate the maximum rectangle area
    let result = max_rectangle_area(&mut points, n);
    println!("{}", result);
}