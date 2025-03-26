use std::cmp::max;
use std::io;
use std::io::Read;

// Helper function to calculate the maximum rectangle area given four points
fn get(points: &[(i32, i32)]) -> i32 {
    let mut max_area = -1;
    for i in 0..(points.len() - 3) {
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

fn max_rectangle_area(points: &mut Vec<Vec<i32>>) -> i32 {
    let mut max_area = -1;
    let n = points.len();

    // Sort the points by their x-coordinates and then by y-coordinates
    points.sort_by(|a, b| {
        if a[0] == b[0] {
            a[1].cmp(&b[1])
        } else {
            a[0].cmp(&b[0])
        }
    });

    // Iterate over the points to find all potential rectangles
    for i in 0..(n - 3) {
        let mut rectangle_points: Vec<(i32, i32)> = Vec::new();

        // Add the first two points of the rectangle
        rectangle_points.push((points[i][0], points[i][1]));
        rectangle_points.push((points[i + 1][0], points[i + 1][1]));

        // Find the next two points that complete the rectangle
        let mut j = i + 2;
        while j < n - 1 {
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
            max_area = max(max_area, get(&rectangle_points));
        }
    }

    max_area
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    // Input the number of points
    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut points: Vec<Vec<i32>> = Vec::with_capacity(n);

    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut nums = line.split_whitespace();
        let x: i32 = nums.next().unwrap().parse().unwrap();
        let y: i32 = nums.next().unwrap().parse().unwrap();
        points.push(vec![x, y]);
    }

    // Call maxRectangleArea and output the result
    let result = max_rectangle_area(&mut points);

    println!("{}", result);

    Ok(())
}