use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::collections::VecDeque;

struct Solution;

impl Solution {
    // Function to calculate maximum distance
    fn max_distance(side: i32, points: Vec<Vec<i32>>, k: usize) -> i32 {
        let n = points.len();
        
        // Calculate a unique order for points around the square
        let ord = |x: i64, y: i64| -> i64 {
            let s = side as i64;
            if y == 0 {
                x
            } else if x == s {
                s + y
            } else if y == s {
                s * 3 - x
            } else {
                s * 4 - y
            }
        };

        // Sort points according to their order around the square
        let mut sorted_points = points.clone();
        sorted_points.sort_by(|a, b| {
            let ord_a = ord(a[0] as i64, a[1] as i64);
            let ord_b = ord(b[0] as i64, b[1] as i64);
            ord_a.cmp(&ord_b)
        });

        // Calculate Manhattan distance between two points
        let dis = |i: usize, j: usize| -> i32 {
            (sorted_points[i][0] - sorted_points[j][0]).abs() + (sorted_points[i][1] - sorted_points[j][1]).abs()
        };

        // Check function for binary search
        let check = |lim: i32| -> bool {
            let mut vec = vec![0];
            for i in 1..n {
                if vec.len() >= k {
                    break;
                }
                if dis(i, vec[vec.len() - 1]) >= lim {
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
                    while vec[j - 1] < n * 2 && dis(vec[j] % n, vec[j - 1] % n) < lim {
                        vec[j] += 1;
                        if vec[j] >= n * 2 {
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

        // Binary search for the largest minimum distance
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

fn main() {
    // Parse input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line containing side, n, K
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_split = first_line.split_whitespace();
    let side: i32 = first_line_split.next().unwrap().parse().unwrap();
    let n: usize = first_line_split.next().unwrap().parse().unwrap();
    let k: usize = first_line_split.next().unwrap().parse().unwrap();

    // Read the points
    let mut points = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut split_line = line.split_whitespace();
        let x: i32 = split_line.next().unwrap().parse().unwrap();
        let y: i32 = split_line.next().unwrap().parse().unwrap();
        points.push(vec![x, y]);
    }

    // Solve the problem
    let solution = Solution;
    let result = solution.max_distance(side, points, k);

    // Output the result
    println!("{}", result);
}