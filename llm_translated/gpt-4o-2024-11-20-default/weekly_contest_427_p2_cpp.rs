use std::cmp::max;
use std::io;

#[derive(Debug)]
struct Solution;

impl Solution {
    // Helper function to calculate the maximum rectangle area given four points
    fn get(points: &Vec<(i32, i32)>) -> i32 {
        let mut max_area = -1;
        for i in 0..points.len() - 3 {
            // Check if these points can form a rectangle:
            // - First and second points have the same x-coordinate
            // - Third and fourth points have the same x-coordinate
            // - First and third points have the same y-coordinate
            // - Second and fourth points have the same y-coordinate
            if points[i].0 == points[i + 1].0
                && points[i + 2].0 == points[i + 3].0
                && points[i].1 == points[i + 2].1
                && points[i + 1].1 == points[i + 3].1
            {
                // Calculate the area of the rectangle
                let area = (points[i + 2].0 - points[i].0) * (points[i + 1].1 - points[i].1);
                max_area = max(max_area, area);
            }
        }
        max_area
    }

    fn max_rectangle_area(&self, points: Vec<Vec<i32>>) -> i32 {
        let mut max_area = -1;
        let n = points.len();

        // Flatten the array of points into a vector of tuples for easier handling.
        let mut sorted_points: Vec<(i32, i32)> = points
            .into_iter()
            .map(|point| (point[0], point[1]))
            .collect();

        // Sort the points by their x-coordinates and then by y-coordinates
        sorted_points.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

        // Iterate over the points to find all potential rectangles
        for i in 0..n - 3 {
            let mut rectangle_points = Vec::with_capacity(4);

            // Add the first two points of the rectangle
            rectangle_points.push(sorted_points[i]);
            rectangle_points.push(sorted_points[i + 1]);

            // Find the next two points that complete the rectangle
            let mut j = i + 2;
            while j < n - 2 {
                // Skip points that don't align with the y-coordinates of the rectangle
                if sorted_points[j].1 > sorted_points[i + 1].1
                    || sorted_points[j].1 < sorted_points[i].1
                {
                    j += 1;
                } else {
                    break;
                }
            }

            // Add the potential third and fourth points
            if j < n - 1 {
                rectangle_points.push(sorted_points[j]);
                rectangle_points.push(sorted_points[j + 1]);

                // Calculate the maximum area for this set of points
                max_area = max(max_area, Self::get(&rectangle_points));
            }
        }

        max_area
    }
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();

    // Read the number of points
    stdin.read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid number format");

    let mut points = Vec::new();

    for _ in 0..n {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read input");
        let point: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid number format"))
            .collect();
        points.push(point);
    }

    // Use the solution to calculate the maximum rectangle area
    let solution = Solution;
    let result = solution.max_rectangle_area(points);

    // Output the result
    println!("{}", result);
}