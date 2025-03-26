use std::cmp::Ordering;

struct Solution;

impl Solution {
    // Helper function to calculate the maximum rectangle area given four points
    fn get(rectangle_points: &[(i32, i32)]) -> i32 {
        if rectangle_points.len() != 4 {
            return -1;
        }

        let p0 = rectangle_points[0];
        let p1 = rectangle_points[1];
        let p2 = rectangle_points[2];
        let p3 = rectangle_points[3];

        // Check the conditions for forming a valid rectangle
        if p0.0 == p1.0 && p2.0 == p3.0 && p0.1 == p2.1 && p1.1 == p3.1 {
            let width = p2.0 - p0.0;
            let height = p1.1 - p0.1;
            width * height
        } else {
            -1
        }
    }

    // Main function to find the maximum rectangle area
    fn max_rectangle_area(points: &mut Vec<(i32, i32)>) -> i32 {
        // Sort points lexicographically (x then y)
        points.sort_unstable();

        let n = points.len();
        let mut max_area = -1;

        // Iterate through potential starting points
        for i in 0..n.saturating_sub(3) {
            let p0 = points[i];
            let p1 = points[i + 1];

            // Find the next valid pair of points
            let mut j = i + 2;
            while j < n.saturating_sub(2) {
                // Check if y-coordinate is within the required range
                match points[j].1.cmp(&p0.1) {
                    Ordering::Less | Ordering::Greater => j += 1,
                    Ordering::Equal => break,
                }
            }

            // Check if valid indices and calculate area
            if j < n.saturating_sub(1) {
                let p2 = points[j];
                let p3 = points[j + 1];
                let current_area = Self::get(&[p0, p1, p2, p3]);
                max_area = max_area.max(current_area);
            }
        }

        max_area
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    // Read number of points
    let mut n_str = String::new();
    stdin_lock.read_line(&mut n_str).unwrap();
    let n: usize = n_str.trim().parse().unwrap();

    // Read all points
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        stdin_lock.read_line(&mut line).unwrap();
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        points.push((parts[0], parts[1]));
    }

    // Calculate and print result
    let result = Solution::max_rectangle_area(&mut points);
    println!("{}", result);
}