use std::cmp::Ordering;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line: side, n, K
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let side: i32 = first_line_iter.next().unwrap().parse().unwrap();
    let n: usize = first_line_iter.next().unwrap().parse().unwrap();
    let k: usize = first_line_iter.next().unwrap().parse().unwrap();

    // Read the next n lines: points
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        points.push(vec![x, y]);
    }

    let sol = Solution;
    println!("{}", sol.max_distance(side, &points, k));
}

struct Solution;

impl Solution {
    pub fn max_distance(&self, side: i32, points: &Vec<Vec<i32>>, k: usize) -> i32 {
        let n = points.len();

        // Define the ordering function
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

        // Sort the points based on the custom ordering
        let mut sorted_points = points.clone();
        sorted_points.sort_by(|a, b| {
            let a_ord = ord(a[0] as i64, a[1] as i64);
            let b_ord = ord(b[0] as i64, b[1] as i64);
            a_ord.cmp(&b_ord)
        });

        // Define the distance function
        let dis = |i: usize, j: usize| -> i32 {
            (sorted_points[i][0] - sorted_points[j][0]).abs() + (sorted_points[i][1] - sorted_points[j][1]).abs()
        };

        // Define the check function
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
                    while dis(vec[j] % n, vec[j - 1] % n) < lim {
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