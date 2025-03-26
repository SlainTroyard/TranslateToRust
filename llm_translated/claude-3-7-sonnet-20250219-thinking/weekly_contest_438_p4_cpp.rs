use std::io;
use std::cmp;

struct Solution;

impl Solution {
    pub fn max_distance(side: i32, mut points: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = points.len();

        // Ordering function to sort points along the perimeter of a square
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

        // Sort points according to the ordering function
        points.sort_by(|a, b| {
            let ord_a = ord(a[0] as i64, a[1] as i64);
            let ord_b = ord(b[0] as i64, b[1] as i64);
            ord_a.cmp(&ord_b)
        });

        // Manhattan distance between two points
        let dis = |i: usize, j: usize| -> i32 {
            (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs()
        };

        // Check if it's possible to select K points with minimum distance >= lim
        let mut check = |lim: i32| -> bool {
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
                    while dis(vec[j] % n, vec[j - 1] % n) < lim {
                        vec[j] += 1;
                        if vec[j] >= n * 2 {
                            return false;
                        }
                    }
                }
                if vec[k as usize - 1] < i + n && dis(i, vec[k as usize - 1] % n) >= lim {
                    return true;
                }
            }
            false
        };

        // Binary search for the maximum minimum distance
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.trim().split_whitespace();
    
    let side: i32 = iter.next().unwrap().parse().expect("Failed to parse side");
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let k: i32 = iter.next().unwrap().parse().expect("Failed to parse K");
    
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read point");
        let coords: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse coordinate"))
            .collect();
        points.push(vec![coords[0], coords[1]]);
    }
    
    // Solve and output
    let solution = Solution::max_distance(side, points, k);
    println!("{}", solution);
}