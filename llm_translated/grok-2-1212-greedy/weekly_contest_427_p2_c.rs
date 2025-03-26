use std::cmp::Ordering;
use std::io::{self, BufRead};

const MAX_POINTS: usize = 100;

// Helper function to calculate the maximum rectangle area given four points
fn get(points: &[[i32; 2]; 4]) -> i32 {
    let mut max_area = -1;
    for i in 0..2 {
        // Check if these points can form a rectangle:
        // - First and second points have the same x-coordinate
        // - Third and fourth points have the same x-coordinate
        // - First and third points have the same y-coordinate
        // - Second and fourth points have the same y-coordinate
        if points[i][0] == points[i + 1][0] &&
           points[i + 2][0] == points[i + 3][0] &&
           points[i][1] == points[i + 2][1] &&
           points[i + 1][1] == points[i + 3][1] {
            // Calculate the area of the rectangle
            let area = (points[i + 2][0] - points[i][0]) * 
                       (points[i + 1][1] - points[i][1]);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

// Comparison function for sorting points by x-coordinate, then y-coordinate
fn compare_points(a: &[i32; 2], b: &[i32; 2]) -> Ordering {
    a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))
}

fn max_rectangle_area(points: &mut [[i32; 2]], n: usize) -> i32 {
    let mut max_area = -1;

    // Sort the points by their x-coordinates and then by y-coordinates
    points.sort_by(compare_points);

    // Iterate over the points to find all potential rectangles
    for i in 0..n - 3 {
        let mut rectangle_points = [[0; 2]; 4];

        // Add the first two points of the rectangle
        rectangle_points[0] = points[i];
        rectangle_points[1] = points[i + 1];

        // Find the next two points that complete the rectangle
        let mut j = i + 2;
        while j < n - 2 {
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of points
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    if n < 4 {
        println!("-1");
        return Ok(());
    }

    // Read the points
    let mut points = [[0; 2]; MAX_POINTS];
    for i in 0..n {
        let line = lines.next().unwrap()?;
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        points[i][0] = nums.next().unwrap();
        points[i][1] = nums.next().unwrap();
    }

    // Calculate and print the result
    let result = max_rectangle_area(&mut points[..n], n);
    println!("{}", result);

    Ok(())
}