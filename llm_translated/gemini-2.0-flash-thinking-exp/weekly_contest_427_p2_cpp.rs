// Problem: Weekly Contest 427 Problem 2
use std::io;
use std::vec::Vec;
use std::cmp;

// Structure to mimic the C++ Solution class (optional, functions work as well)
struct Solution {}

impl Solution {
    // Helper function to calculate the maximum rectangle area given four points
    fn get(points: &Vec<(i32, i32)>) -> i32 {
        let mut max_area = -1;
        for i in 0..(points.len() - 3) {
            // Check if these points can form a rectangle:
            // - First and second points have the same x-coordinate
            // - Third and fourth points have the same x-coordinate
            // - First and third points have the same y-coordinate
            // - Second and fourth points have the same y-coordinate
            if points[i].0 == points[i + 1].0 &&
                points[i + 2].0 == points[i + 3].0 &&
                points[i].1 == points[i + 2].1 &&
                points[i + 1].1 == points[i + 3].1 {
                // Calculate the area of the rectangle
                let area = (points[i + 2].0 - points[i].0) *
                           (points[i + 1].1 - points[i].1);
                max_area = cmp::max(max_area, area);
            }
        }
        max_area
    }

    fn max_rectangle_area(points: &mut Vec<Vec<i32>>) -> i32 {
        let mut max_area = -1;
        let n = points.len();

        // Sort the points by their x-coordinates and then by y-coordinates
        points.sort_by_key(|p| (p[0], p[1]));

        // Iterate over the points to find all potential rectangles
        for i in 0..(n - 3) {
            let mut rectangle_points: Vec<(i32, i32)> = Vec::new();

            // Add the first two points of the rectangle
            rectangle_points.push((points[i][0], points[i][1]));
            rectangle_points.push((points[i + 1][0], points[i + 1][1]));

            // Find the next two points that complete the rectangle
            let mut j = i + 2;
            while j < n - 2 {
                // Skip points that don't align with the y-coordinates of the rectangle
                if points[j][1] > points[i + 1][1] || points[j][1] < points[i][1] {
                    j += 1;
                } else {
                    break;
                }
            }

            // Add the potential third and fourth points
            if j < n - 1 {
                rectangle_points.push((points[j][0], points[j][1]));
                rectangle_points.push((points[j + 1][0], points[j + 1][1]));

                // Calculate the maximum area for this set of points
                max_area = cmp::max(max_area, Self::get(&rectangle_points));
            }
        }

        max_area
    }
}

fn main() {
    let mut solution = Solution {};

    // Input the number of points
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Please type a number!");

    let mut points: Vec<Vec<i32>> = Vec::new();

    for _i in 0..n {
        let mut point_str = String::new();
        io::stdin().read_line(&mut point_str).expect("Failed to read line");
        let coords: Vec<i32> = point_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();
        points.push(coords);
    }

    // Call maxRectangleArea and output the result
    let result = solution.max_rectangle_area(&mut points);

    println!("{}", result);
}