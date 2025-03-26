use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = points.len();

        // Ordering function to sort points along the perimeter
        let ord = |x: i64, y: i64| -> i64 {
            let s = side as i64;
            if y == 0 {
                return x;
            } else if x == s {
                return s + y;
            } else if y == s {
                return s * 3 - x;
            } else {
                return s * 4 - y;
            }
        };

        // Sort points based on their order along the perimeter
        let mut points = points;
        points.sort_by(|a, b| {
            let ord_a = ord(a[0] as i64, a[1] as i64);
            let ord_b = ord(b[0] as i64, b[1] as i64);
            ord_a.cmp(&ord_b)
        });

        // Manhattan distance between two points
        let dis = |i: usize, j: usize| -> i32 {
            (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs()
        };

        // Check if it's possible to select K points with minimum distance 'lim'
        let check = |lim: i32| -> bool {
            let mut vec = vec![0];
            for i in 1..n {
                if vec.len() < k as usize && dis(i, *vec.last().unwrap()) >= lim {
                    vec.push(i);
                }
            }
            if vec.len() < k as usize {
                return false;
            }
            if dis(vec[0], *vec.last().unwrap()) >= lim {
                return true;
            }
            for i in 1..n {
                vec[0] = i;
                for j in 1..k as usize {
                    let mut idx = vec[j];
                    while dis(idx % n, vec[j - 1] % n) < lim {
                        idx += 1;
                        if idx >= n * 2 {
                            return false;
                        }
                    }
                    vec[j] = idx;
                }
                if vec.last().unwrap() < &(i + n) && dis(i, vec.last().unwrap() % n) >= lim {
                    return true;
                }
            }
            false
        };

        // Binary search to find the maximum possible minimum distance
        let mut head = 1;
        let mut tail = side;
        while head < tail {
            let mid = (head + tail + 1) >> 1;
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
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse first line for side, n, and K
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let side: i32 = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Parse points
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        points.push(vec![x, y]);
    }
    
    // Solve and output result
    let sol = Solution::max_distance(side, points, k);
    println!("{}", sol);
}