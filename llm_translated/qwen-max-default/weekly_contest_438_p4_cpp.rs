use std::io::{self, BufRead, Write};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let side = first_line[0];
    let n = first_line[1] as usize;
    let k = first_line[2];

    // Read points
    let mut points: Vec<(i32, i32)> = Vec::with_capacity(n);
    for _ in 0..n {
        let line: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        points.push((line[0], line[1]));
    }

    // Solve the problem
    let result = max_distance(side, &points, k);

    // Write the result to stdout
    println!("{}", result);
}

// Function to calculate the maximum distance
fn max_distance(side: i32, points: &[(i32, i32)], k: i32) -> i32 {
    let n = points.len();

    // Custom ordering function
    let ord = |(x, y): (i32, i32)| {
        let s = side;
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

    // Sort points using the custom ordering
    let mut sorted_points = points.to_vec();
    sorted_points.sort_by_key(|&p| ord(p));

    // Distance calculation function
    let dis = |i: usize, j: usize| {
        (sorted_points[i].0 - sorted_points[j].0).abs() + (sorted_points[i].1 - sorted_points[j].1).abs()
    };

    // Check function to determine if a given limit is valid
    let check = |lim: i32| {
        let mut vec = vec![0];
        for i in 1..n {
            if vec.len() >= k as usize {
                break;
            }
            if dis(i, vec.last().unwrap()) >= lim {
                vec.push(i);
            }
        }
        if vec.len() < k as usize {
            return false;
        }
        if dis(vec[0], vec.last().unwrap()) >= lim {
            return true;
        }
        for i in 1..n {
            if vec.last().unwrap() >= &n * 2 {
                return false;
            }
            vec[0] = i;
            for j in 1..k as usize {
                while dis(vec[j] % n, vec[j - 1] % n) < lim {
                    vec[j] += 1;
                    if vec[j] >= n * 2 {
                        return false;
                    }
                }
            }
            if vec.last().unwrap() < &i + n && dis(i, vec.last().unwrap() % n) >= lim {
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