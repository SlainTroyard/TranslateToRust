use std::io::{self, BufRead, Write};

fn get(points: &[(i32, i32)]) -> i32 {
    let mut max_area = -1;
    for i in 0..points.len() - 3 {
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
            let area = (points[i + 2].0 - points[i].0).abs() * (points[i + 1].1 - points[i].1).abs();
            max_area = max_area.max(area);
        }
    }
    max_area
}

fn max_rectangle_area(points: &mut Vec<(i32, i32)>) -> i32 {
    let mut max_area = -1;
    let n = points.len();

    // Sort the points by their x-coordinates and then by y-coordinates
    points.sort_unstable();

    // Iterate over the points to find all potential rectangles
    for i in 0..n - 3 {
        let mut rectangle_points = vec![];

        // Add the first two points of the rectangle
        rectangle_points.push(points[i]);
        rectangle_points.push(points[i + 1]);

        // Find the next two points that complete the rectangle
        let mut j = i + 2;
        while j < n - 2 {
            // Skip points that don't align with the y-coordinates of the rectangle
            if points[j].1 > points[i + 1].1 || points[j].1 < points[i].1 {
                j += 1;
            } else {
                break;
            }
        }

        // Add the potential third and fourth points
        if j < n - 1 {
            rectangle_points.push(points[j]);
            rectangle_points.push(points[j + 1]);

            // Calculate the maximum area for this set of points
            max_area = max_area.max(get(&rectangle_points));
        }
    }

    max_area
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();

    // Input the number of points
    let mut input = String::new();
    stdin_lock.read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    let mut points: Vec<(i32, i32)> = Vec::with_capacity(n);

    for _ in 0..n {
        input.clear();
        stdin_lock.read_line(&mut input).expect("Failed to read line");
        let coords: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Please type a number!"))
            .collect();
        points.push((coords[0], coords[1]));
    }

    // Call max_rectangle_area and output the result
    let result = max_rectangle_area(&mut points);

    writeln!(stdout_lock, "{}", result).expect("Failed to write to stdout");
}