/// This Rust program is an idiomatic translation of the provided C++ code
/// for LeetCode Weekly Contest 438 Problem 4. It preserves the exact same
/// algorithm and I/O format as the original C++ solution.
///
/// To use:
/// 1. Compile and run this program.
/// 2. Provide the input in the format:
///      side n K
///      x1 y1
///      x2 y2
///      ...
///      xn yn
/// 3. The output will be the single integer result printed to stdout.

use std::io::{self, BufRead};

struct Solution;

impl Solution {
    /// Translated function from C++: Solution::maxDistance(int side, vector<vector<int>> &points, int K)
    /// side: length of each side of the square
    /// points: a mutable reference to the set of points on the perimeter of the square
    /// K: number of points to be selected
    fn max_distance(&self, side: i32, points: &mut Vec<[i32; 2]>, k: i32) -> i32 {
        let n = points.len();

        // This closure replicates the 'ord' function from the C++ code
        // Calculates a value that determines how a point (x, y) on the perimeter is "ordered"
        // around the square perimeter.
        let ord = |x: i32, y: i32| -> i64 {
            let s = side as i64;
            if y == 0 {
                x as i64
            } else if x as i64 == s {
                s + y as i64
            } else if y as i64 == s {
                s * 3 - x as i64
            } else {
                s * 4 - y as i64
            }
        };

        // Sort points in the same way as in the C++ code
        points.sort_by_key(|p| ord(p[0], p[1]));

        // Distance function equivalent to the 'dis' lambda in C++
        let dis = |i: usize, j: usize| -> i32 {
            (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs()
        };

        // The 'check' closure replicates the logic of checking if there's a way
        // to pick K points such that minimum pairwise distance is at least `lim`.
        let check = |lim: i32| -> bool {
            let mut vec = vec![0];
            // First pass: try to build a chain of points (indices) with distance >= lim
            for i in 1..n {
                if vec.len() < k as usize {
                    if dis(i, *vec.last().unwrap()) >= lim {
                        vec.push(i);
                    }
                } else {
                    break;
                }
            }
            // If we cannot pick k points, return false immediately
            if vec.len() < k as usize {
                return false;
            }
            // If the distance between the first and last in this chain is >= lim, success
            if dis(vec[0], *vec.last().unwrap()) >= lim {
                return true;
            }

            // Second pass: try shifting the chain's starting index
            // while reassigning subsequent points to see if there's a configuration that satisfies lim
            for i in 1..n {
                // We cut off if we've already moved beyond 2*n for the last point
                if *vec.last().unwrap() >= (n as i32 * 2) as usize {
                    break;
                }
                vec[0] = i; // Reassign the first point
                // Reassign each of the remaining k-1 indices
                for j in 1..k as usize {
                    // Increase index until distance requirement is met or we exceed 2*n
                    while dis(vec[j] % n, vec[j - 1] % n) < lim {
                        vec[j] += 1;
                        if vec[j] >= 2 * n {
                            return false;
                        }
                    }
                }
                // If the last point in vec is within i + n, and it satisfies the distance from i
                if vec[k as usize - 1] < i + n
                    && dis(i, vec[k as usize - 1] % n) >= lim
                {
                    return true;
                }
            }
            false
        };

        // Binary search for the maximum distance
        let mut head = 1;
        let mut tail = side;
        while head < tail {
            let mid = (head + tail + 1) >> 1; // (head + tail + 1) / 2
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
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read side, n, K (all on one line according to the C++ code)
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.trim().split_whitespace();
    let side: i32 = parts.next().unwrap().parse().unwrap();
    let n: i32 = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    // Read n lines, each containing two integers (the coordinates of each point)
    let mut points = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut coords = line.trim().split_whitespace();
        let x: i32 = coords.next().unwrap().parse().unwrap();
        let y: i32 = coords.next().unwrap().parse().unwrap();
        points.push([x, y]);
    }

    // Create an instance of our solution and compute the answer
    let sol = Solution;
    let answer = sol.max_distance(side, &mut points, k);

    // Print the result to stdout
    println!("{}", answer);
}