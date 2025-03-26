use std::io::{self, BufRead};

/// Calculates the maximum area of a rectangle from the given list of points.
/// The function expects a slice of points (each point is [x, y]) and the size of that slice.
/// It iterates over these points (in a sliding window of 4) and checks if they form a rectangle
/// according to the following conditions:
///   - points[i] and points[i+1] share the same x-coordinate
///   - points[i+2] and points[i+3] share the same x-coordinate
///   - points[i] and points[i+2] share the same y-coordinate
///   - points[i+1] and points[i+3] share the same y-coordinate
/// If the condition is met, it computes the area. Otherwise, it continues.
/// Returns the maximum area found, or -1 if no rectangle is found.
fn get(points: &[[i32; 2]], size: usize) -> i32 {
    let mut max_area = -1;
    // Loop from i = 0 to (size - 3)
    for i in 0..(size - 3) {
        // Check if these 4 points can form a rectangle
        if points[i][0] == points[i + 1][0] &&
           points[i + 2][0] == points[i + 3][0] &&
           points[i][1] == points[i + 2][1] &&
           points[i + 1][1] == points[i + 3][1] {
            let area = (points[i + 2][0] - points[i][0]) * (points[i + 1][1] - points[i][1]);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

/// Given a sorted list of points, this function finds the maximum rectangle area.
/// It tries every possible group of 4 points (following similar logic as in the original C code)
/// by first selecting two consecutive points and then searching for the other two points that
/// complete the rectangle based on their y coordinates. Returns the maximum area or -1 if no valid
/// rectangle is found.
fn max_rectangle_area(points: &[[i32; 2]]) -> i32 {
    let n = points.len();
    let mut max_area = -1;
    // Iterate over all possible starting indices for a potential rectangle
    for i in 0..(n.saturating_sub(3)) {
        // Create an array to hold 4 points that may form a rectangle.
        let mut rectangle_points = [[0; 2]; 4];
        // The first two points are taken directly from the points list.
        rectangle_points[0] = points[i];
        rectangle_points[1] = points[i + 1];

        // Find the next two points that may form the rectangle.
        // We look for the next point with y coordinate between points[i] and points[i+1]
        let mut j = i + 2;
        while j < n - 2 {
            if points[j][1] > points[i + 1][1] || points[j][1] < points[i][1] {
                j += 1;
            } else {
                break;
            }
        }

        // We need at least two points to complete a rectangle.
        if j < n - 1 {
            rectangle_points[2] = points[j];
            rectangle_points[3] = points[j + 1];

            // Use the helper function get() to compute rectangle area from these 4 points.
            let area = get(&rectangle_points, 4);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

/// Main function to handle I/O and execute the algorithm.
/// It reads an integer n denoting the number of points.
/// If n < 4, it prints "-1" and exits.
/// Otherwise, it reads n points (each point is given as two space-separated integers),
/// sorts them by x-coordinate first then by y-coordinate (as in the original C code),
/// computes the maximum rectangle area, and prints the result.
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    // Read the first line (number of points)
    reader.read_line(&mut input)?;
    let n: usize = input.trim().parse().unwrap_or(0);
    
    // If there are fewer than 4 points, output -1 and exit as per problem specification.
    if n < 4 {
        println!("-1");
        return Ok(());
    }
    
    // Clear input buffer for next lines
    input.clear();
    let mut points: Vec<[i32; 2]> = Vec::with_capacity(n);

    // Read points from standard input. The points may be on separate lines or on the same line.
    // We'll split the entire input by whitespace.
    let mut tokens: Vec<String> = Vec::new();
    while reader.read_line(&mut input)? > 0 {
        tokens.extend(input.split_whitespace().map(|s| s.to_string()));
        input.clear();
    }

    // Check that we have enough tokens to form n points (each point consists of 2 integers)
    if tokens.len() < n * 2 {
        eprintln!("Expected {} points, but got fewer tokens.", n);
        std::process::exit(1);
    }

    // Parse tokens into points.
    for i in 0..n {
        let idx = i * 2;
        let x: i32 = tokens[idx].parse().unwrap();
        let y: i32 = tokens[idx + 1].parse().unwrap();
        points.push([x, y]);
    }

    // Sort the points by x-coordinate, then by y-coordinate.
    points.sort_by(|a, b| {
        if a[0] != b[0] {
            a[0].cmp(&b[0])
        } else {
            a[1].cmp(&b[1])
        }
    });

    // Calculate the maximum rectangle area.
    let result = max_rectangle_area(&points);
    println!("{}", result);

    Ok(())
}