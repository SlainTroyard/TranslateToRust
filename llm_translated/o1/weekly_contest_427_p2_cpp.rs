// LeetCode Weekly Contest 427 Problem 2
// Translated from C++ to Rust

use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Helper function to calculate the maximum rectangle area given four points.
    // Each point is represented as (x, y).
    fn get(&self, points: &Vec<(i32, i32)>) -> i32 {
        let mut max_area = -1;
        // We need at least four points to form a rectangle, so we iterate up to points.len() - 3
        for i in 0..points.len().saturating_sub(3) {
            // Check if these points can form a rectangle:
            // 1) points[i] and points[i+1] share the same x-coordinate
            // 2) points[i+2] and points[i+3] share the same x-coordinate
            // 3) points[i] and points[i+2] share the same y-coordinate
            // 4) points[i+1] and points[i+3] share the same y-coordinate
            if points[i].0 == points[i + 1].0
                && points[i + 2].0 == points[i + 3].0
                && points[i].1 == points[i + 2].1
                && points[i + 1].1 == points[i + 3].1
            {
                // Calculate the area of the rectangle
                let area = (points[i + 2].0 - points[i].0) * (points[i + 1].1 - points[i].1);
                if area > max_area {
                    max_area = area;
                }
            }
        }
        max_area
    }

    // Main function to find the maximum rectangle area from a list of points.
    fn max_rectangle_area(&self, points: &mut Vec<[i32; 2]>) -> i32 {
        let mut max_area = -1;
        let n = points.len();

        // Sort the points by their x-coordinate, then by their y-coordinate
        points.sort();

        // Iterate over the points to explore potential rectangles
        for i in 0..n.saturating_sub(3) {
            let mut rectangle_points = Vec::new();

            // Add the first two points of a potential rectangle
            rectangle_points.push((points[i][0], points[i][1]));
            rectangle_points.push((points[i + 1][0], points[i + 1][1]));

            // Find the next two points that might complete the rectangle
            let mut j = i + 2;
            while j < n.saturating_sub(2) {
                // Skip points that don't align with the y-coordinates of the first two points
                if points[j][1] > points[i + 1][1] || points[j][1] < points[i][1] {
                    j += 1;
                } else {
                    break;
                }
            }

            // If we still have enough points to form four corners
            if j < n.saturating_sub(1) {
                rectangle_points.push((points[j][0], points[j][1]));
                rectangle_points.push((points[j + 1][0], points[j + 1][1]));

                // Calculate the maximum area for this set of points
                let area = self.get(&rectangle_points);
                if area > max_area {
                    max_area = area;
                }
            }
        }

        max_area
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of points
    let n = lines
        .next()
        .unwrap()
        .expect("Failed to read number of points")
        .trim()
        .parse::<usize>()
        .expect("Failed to parse number of points");

    // Read each point's x and y
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines
            .next()
            .unwrap()
            .expect("Failed to read point coordinates");
        let mut iter = line.split_whitespace();
        let x = iter
            .next()
            .expect("Missing x-coordinate")
            .parse::<i32>()
            .expect("Failed to parse x-coordinate");
        let y = iter
            .next()
            .expect("Missing y-coordinate")
            .parse::<i32>()
            .expect("Failed to parse y-coordinate");
        points.push([x, y]);
    }

    // Create a Solution instance and calculate the maximum rectangle area
    let solution = Solution;
    let result = solution.max_rectangle_area(&mut points);

    // Output the result
    println!("{}", result);
}