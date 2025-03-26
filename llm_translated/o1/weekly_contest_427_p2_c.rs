use std::io::{self, BufRead};

// Helper function to calculate the maximum rectangle area given four points
// It checks if these four points form a rectangle based on the conditions:
// 1) First and second points have the same x-coordinate
// 2) Third and fourth points have the same x-coordinate
// 3) First and third points have the same y-coordinate
// 4) Second and fourth points have the same y-coordinate
// Then it returns the maximum area found among all 4-point combinations in the slice.
fn get(points: &[[i32; 2]]) -> i32 {
    let size = points.len();
    let mut max_area = -1;
    for i in 0..(size - 3) {
        if points[i][0] == points[i + 1][0]
            && points[i + 2][0] == points[i + 3][0]
            && points[i][1] == points[i + 2][1]
            && points[i + 1][1] == points[i + 3][1]
        {
            let area = (points[i + 2][0] - points[i][0])
                * (points[i + 1][1] - points[i][1]);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

// Calculates the maximum rectangle area from the given points by:
// 1) Sorting the points by their x-coordinate, then by y-coordinate
// 2) Iterating over the points to find potential rectangles
// 3) Checking the next two points that complete the rectangle
//    (by ensuring their y-coordinates fit between the y-range of the first two points)
// 4) Using the helper function get(...) to compute the rectangle area if valid
fn max_rectangle_area(points: &mut [[i32; 2]]) -> i32 {
    // Sort by x, then by y
    points.sort_by(|a, b| {
        if a[0] != b[0] {
            a[0].cmp(&b[0])
        } else {
            a[1].cmp(&b[1])
        }
    });

    let n = points.len();
    let mut max_area = -1;

    // Iterate over possible 'starting' indices for rectangles
    for i in 0..(n - 3) {
        // Collect the first two points
        let mut rectangle_points = [[0; 2]; 4];
        rectangle_points[0] = points[i];
        rectangle_points[1] = points[i + 1];

        // Find the next two points that complete the rectangle
        let mut j = i + 2;
        while j < n - 2 {
            // Skip points whose y-coordinates are out of the range
            // of the first two points' y-coordinates
            if points[j][1] > points[i + 1][1]
                || points[j][1] < points[i][1]
            {
                j += 1;
            } else {
                break;
            }
        }

        // If we have enough points left to form a rectangle, compute the area
        if j < n - 1 {
            rectangle_points[2] = points[j];
            rectangle_points[3] = points[j + 1];

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
    // Read all lines and split them into tokens to mimic scanf behavior
    let lines: Vec<String> = stdin.lock().lines()
        .map(|l| l.unwrap_or_default())
        .collect();
    let mut tokens = Vec::new();
    for line in lines {
        tokens.extend(line.split_whitespace().map(String::from));
    }

    // If no tokens were read, print "-1" (since we cannot proceed) and return
    if tokens.is_empty() {
        println!("-1");
        return;
    }

    // Parse n
    let n: i32 = match tokens[0].parse() {
        Ok(val) => val,
        Err(_) => {
            println!("-1");
            return;
        }
    };

    // If fewer than 4 points, immediately output -1
    if n < 4 {
        println!("-1");
        return;
    }

    // Check if we have enough tokens to read n pairs of coordinates
    if tokens.len() < (1 + 2 * n as usize) {
        println!("-1");
        return;
    }

    // Read the points
    let mut points = Vec::with_capacity(n as usize);
    for i in 0..n as usize {
        let x = match tokens[1 + 2 * i].parse::<i32>() {
            Ok(val) => val,
            Err(_) => {
                println!("-1");
                return;
            }
        };
        let y = match tokens[2 + 2 * i].parse::<i32>() {
            Ok(val) => val,
            Err(_) => {
                println!("-1");
                return;
            }
        };
        points.push([x, y]);
    }

    // Calculate and print the result
    let result = max_rectangle_area(&mut points);
    println!("{}", result);
}