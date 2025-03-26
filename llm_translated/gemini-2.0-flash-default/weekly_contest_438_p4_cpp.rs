use std::io;
use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn max_distance(&self, side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = points.len();
        let k = k as usize; // Convert k to usize

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

        let mut mutable_points = points.clone(); // Create a mutable copy of points

        mutable_points.sort_by(|a, b| {
            ord(a[0] as i64, a[1] as i64).cmp(&ord(b[0] as i64, b[1] as i64))
        });

        let dis = |i: usize, j: usize| -> i32 {
            (mutable_points[i][0] - mutable_points[j][0]).abs() + (mutable_points[i][1] - mutable_points[j][1]).abs()
        };

        let check = |lim: i32| -> bool {
            let mut vec: Vec<usize> = vec![0];
            let mut i = 1;
            while i < n && vec.len() < k {
                if dis(i, *vec.last().unwrap()) >= lim {
                    vec.push(i);
                }
                i += 1;
            }

            if vec.len() < k {
                return false;
            }

            if dis(vec[0], *vec.last().unwrap()) >= lim {
                return true;
            }

            let mut i = 1;
            while i < n && *vec.last().unwrap() < n * 2 {
                vec[0] = i;

                let mut j = 1;
                while j < k {
                    while dis(vec[j] % n, vec[j - 1] % n) < lim {
                        vec[j] += 1;
                        if vec[j] >= n * 2 {
                            return false;
                        }
                    }
                    j += 1;
                }

                if *vec.last().unwrap() < i + n && dis(i, vec.last().unwrap() % n) >= lim {
                    return true;
                }
                i += 1;
            }
            false
        };

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
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut nums = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap());

    let side: i32 = nums.next().unwrap();
    let n: i32 = nums.next().unwrap();
    let k: i32 = nums.next().unwrap();

    let mut points: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let point: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        points.push(point);
    }

    let sol = Solution {};
    println!("{}", sol.max_distance(side, points, k));
}