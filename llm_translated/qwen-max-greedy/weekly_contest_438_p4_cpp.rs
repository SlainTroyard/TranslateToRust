use std::io::{self, BufRead, Write};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line for side, n, and K
    let first_line = lines.next().unwrap().expect("Failed to read the first line");
    let mut iter = first_line.split_whitespace();
    let side: i32 = iter.next().unwrap().parse().expect("Failed to parse side");
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let k: usize = iter.next().unwrap().parse().expect("Failed to parse K");

    // Parse the points
    let mut points: Vec<(i32, i32)> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().expect("Failed to read a line");
        let mut iter = line.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().expect("Failed to parse x");
        let y: i32 = iter.next().unwrap().parse().expect("Failed to parse y");
        points.push((x, y));
    }

    // Create an instance of Solution and compute the result
    let sol = Solution;
    let result = sol.max_distance(side, &points, k);

    // Write the result to stdout
    println!("{}", result);
}

struct Solution;

impl Solution {
    fn max_distance(&self, side: i32, points: &[(i32, i32)], k: usize) -> i32 {
        let n = points.len();

        // Define the ordering function
        let ord = |(x, y): (i32, i32)| {
            if y == 0 {
                x
            } else if x == side {
                side + y
            } else if y == side {
                3 * side - x
            } else {
                4 * side - y
            }
        };

        // Sort the points based on the ordering function
        let mut sorted_points = points.to_vec();
        sorted_points.sort_by_key(|&p| ord(p));

        // Define the distance function
        let dis = |i: usize, j: usize| {
            (sorted_points[i].0 - sorted_points[j].0).abs() + (sorted_points[i].1 - sorted_points[j].1).abs()
        };

        // Define the check function
        let check = |lim: i32| {
            let mut vec = vec![0];
            for i in 1..n {
                if vec.len() < k && dis(i, vec[vec.len() - 1]) >= lim {
                    vec.push(i);
                }
            }
            if vec.len() < k {
                return false;
            }
            if dis(vec[0], vec[vec.len() - 1]) >= lim {
                return true;
            }
            for i in 1..n {
                vec[0] = i;
                for j in 1..k {
                    while dis(vec[j % n], vec[(j - 1) % n]) < lim {
                        vec[j] += 1;
                        if vec[j] >= 2 * n {
                            return false;
                        }
                    }
                }
                if vec[vec.len() - 1] < i + n && dis(i, vec[vec.len() - 1] % n) >= lim {
                    return true;
                }
            }
            false
        };

        // Binary search for the maximum distance
        let mut head = 1;
        let mut tail = side;
        while head < tail {
            let mid = (head + tail + 1) / 2;
            if check(mid) {
                head = mid;
            } else {
                tail = mid - 1;
            }
        }
        head
    }
}