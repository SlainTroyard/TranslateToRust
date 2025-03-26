use std::io::{self, BufRead};
use std::cmp;

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
        let area = (points[2][0] - points[0][0]) * 
                   (points[1][1] - points[0][1]);
        if area > max_area {
            max_area = area;
        }
    }
    
    max_area
}

fn max_rectangle_area(points: &mut [[i32; 2]], n: usize) -> i32 {
    let mut max_area = -1;

    // Sort the points by their x-coordinates and then by y-coordinates
    points[0..n].sort_by(|a, b| {
        if a[0] != b[0] {
            a[0].cmp(&b[0])
        } else {
            a[1].cmp(&b[1])
        }
    });

    // Iterate over the points to find all potential rectangles
    for i in 0..n.saturating_sub(3) {
        let mut rectangle_points = [[0; 2]; 4];

        // Add the first two points of the rectangle
        rectangle_points[0][0] = points[i][0];
        rectangle_points[0][1] = points[i][1];
        rectangle_points[1][0] = points[i + 1][0];
        rectangle_points[1][1] = points[i + 1][1];

        // Find the next two points that complete the rectangle
        let mut j = i + 2;
        while j < n.saturating_sub(2) {
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
            max_area = cmp::max(max_area, area);
        }
    }

    max_area
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of points
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    if n < 4 {
        println!("-1");
        return;
    }

    let mut points = [[0; 2]; MAX_POINTS];
    
    // Read the coordinates of each point
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        points[i][0] = coords[0];
        points[i][1] = coords[1];
    }

    let result = max_rectangle_area(&mut points, n);
    println!("{}", result);
}