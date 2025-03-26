use std::io;

struct Solution;

impl Solution {
    // Helper function to calculate the maximum rectangle area given four points
    fn get(points: &Vec<(i32, i32)>) -> i32 {
        let mut max_area = -1;
        for i in 0..points.len() - 3 {
            if points[i].0 == points[i + 1].0
                && points[i + 2].0 == points[i + 3].0
                && points[i].1 == points[i + 2].1
                && points[i + 1].1 == points[i + 3].1
            {
                let area = (points[i + 2].0 - points[i].0) * (points[i + 1].1 - points[i].1);
                if area > max_area {
                    max_area = area;
                }
            }
        }
        max_area
    }

    // Main function to find the maximum rectangle area
    pub fn max_rectangle_area(points: &mut Vec<(i32, i32)>) -> i32 {
        let mut max_area = -1;
        let n = points.len();

        // Sort the points by x-coordinate, then by y-coordinate
        points.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

        for i in 0..n - 3 {
            let mut rectangle_points = Vec::new();
            rectangle_points.push(points[i]);
            rectangle_points.push(points[i + 1]);

            let mut j = i + 2;
            while j < n - 2 {
                if points[j].1 > points[i + 1].1 || points[j].1 < points[i].1 {
                    j += 1;
                } else {
                    break;
                }
            }

            if j < n - 1 {
                rectangle_points.push(points[j]);
                rectangle_points.push(points[j + 1]);
                let current_area = Self::get(&rectangle_points);
                if current_area > max_area {
                    max_area = current_area;
                }
            }
        }

        max_area
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        points.push((x, y));
    }

    let result = Solution::max_rectangle_area(&mut points);
    println!("{}", result);
}