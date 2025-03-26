use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    // Helper function to calculate the maximum rectangle area given four points
    // The points are stored as a vector of (x, y) pairs.
    // We iterate over possible quadruplets in the vector (in order) and check if they form a rectangle:
    // - first two points: same x coordinate
    // - next two points: same x coordinate
    // - first and third: same y coordinate
    // - second and fourth: same y coordinate
    // If they do form a rectangle, compute the area.
    fn get(points: &[ (i32, i32) ]) -> i32 {
        let mut max_area: i32 = -1;
        // We need at least 4 points, so iterate with index from 0 to len-3
        for i in 0..points.len() - 3 {
            if points[i].0 == points[i + 1].0 &&
               points[i + 2].0 == points[i + 3].0 &&
               points[i].1 == points[i + 2].1 &&
               points[i + 1].1 == points[i + 3].1
            {
                let area = (points[i + 2].0 - points[i].0) *
                           (points[i + 1].1 - points[i].1);
                max_area = max(max_area, area);
            }
        }
        max_area
    }

    // Main function to find the maximum rectangle area from the given list of points.
    // The input "points" is a vector of vectors, where each inner vector has exactly 2 elements.
    // We follow the same logic as the C++ code.
    fn max_rectangle_area(&self, mut points: Vec<Vec<i32>>) -> i32 {
        let mut max_area: i32 = -1;
        let n = points.len();
        
        // Sort the points lexicographically: first by x, then by y.
        points.sort();
        
        // Iterate over the points to find all potential rectangles.
        for i in 0..(n.saturating_sub(3)) {
            let mut rectangle_points: Vec<(i32, i32)> = Vec::with_capacity(4);
            
            // Add the first two points of the rectangle.
            rectangle_points.push((points[i][0], points[i][1]));
            rectangle_points.push((points[i + 1][0], points[i + 1][1]));
            
            // Find the next two points that complete the rectangle
            let mut j = i + 2;
            while j < n.saturating_sub(2) {
                // Skip points that don't align with the y-coordinates of the rectangle.
                if points[j][1] > points[i + 1][1] || points[j][1] < points[i][1] {
                    j += 1;
                } else {
                    break;
                }
            }
            
            // If we have a valid j that can supply two more points
            if j < n.saturating_sub(1) {
                rectangle_points.push((points[j][0], points[j][1]));
                rectangle_points.push((points[j + 1][0], points[j + 1][1]));
                
                // Calculate the maximum area for this set of points.
                max_area = max(max_area, Self::get(&rectangle_points));
            }
        }
        
        max_area
    }
}

fn main() -> io::Result<()> {
    // Use a buffered reader for efficient input.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input_line = String::new();
    
    // Read first integer representing number of points.
    reader.read_line(&mut input_line)?;
    let n: usize = input_line.trim().parse().expect("Invalid number");
    
    // Prepare the vector for points.
    let mut points: Vec<Vec<i32>> = Vec::with_capacity(n);
    // Read each line with two integers.
    for _ in 0..n {
        input_line.clear();
        reader.read_line(&mut input_line)?;
        let nums: Vec<i32> = input_line
            .split_whitespace()
            .map(|num_str| num_str.parse().expect("Invalid integer"))
            .collect();
        // Expect exactly 2 integers per line
        if nums.len() != 2 {
            panic!("Each point must consist of two integers.");
        }
        points.push(nums);
    }
    
    // Create the solution instance and compute the maximum rectangle area.
    let solution = Solution;
    let result = solution.max_rectangle_area(points);
    
    // Print the result to stdout.
    println!("{}", result);
    
    Ok(())
}