use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_distance(&self, side: i32, points: &mut Vec<Vec<i32>>, k: i32) -> i32 {
        let n = points.len();
        let k = k as usize;
        let side = side as i64;

        // Sort points based on their order around the square
        points.sort_by(|a, b| {
            let ord = |x: i32, y: i32| -> i64 {
                let (x, y) = (x as i64, y as i64);
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
            ord(a[0], a[1]).cmp(&ord(b[0], b[1]))
        });

        // Manhattan distance closure
        let dis = |i: usize, j: usize| -> i32 {
            (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs()
        };

        // Binary search for maximum valid distance
        let mut head = 1;
        let mut tail = side as i32;
        while head < tail {
            let mid = (head + tail + 1) / 2;
            let mut valid = false;
            
            // First phase: try to collect k points with minimum spacing
            let mut vec = vec![0];
            for i in 1..n {
                if vec.len() >= k {
                    break;
                }
                if dis(i, *vec.last().unwrap()) >= mid {
                    vec.push(i);
                }
            }
            
            if vec.len() >= k {
                // Check if initial sequence wraps around
                if dis(vec[0], vec[k-1]) >= mid {
                    valid = true;
                } else {
                    // Second phase: try different starting points
                    'outer: for start in 1..n {
                        let mut current_vec = vec.clone();
                        current_vec[0] = start;
                        
                        for j in 1..k {
                            while {
                                let prev = current_vec[j-1] % n;
                                let curr = current_vec[j] % n;
                                dis(prev as usize, curr as usize) < mid
                            } {
                                current_vec[j] += 1;
                                if current_vec[j] >= 2 * n as i32 {
                                    break 'outer;
                                }
                            }
                        }
                        
                        if current_vec[k-1] < start + n as i32 
                            && dis(start as usize, (current_vec[k-1] % n as i32) as usize) >= mid 
                        {
                            valid = true;
                            break;
                        }
                    }
                }
            }
            
            if valid {
                head = mid;
            } else {
                tail = mid - 1;
            }
        }
        
        head
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();
    
    // Read first line containing side, n, k
    stdin.read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let side: i32 = parts.next().unwrap().parse().unwrap();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    
    // Read point coordinates
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let mut coords = line.split_whitespace();
        let x: i32 = coords.next().unwrap().parse().unwrap();
        let y: i32 = coords.next().unwrap().parse().unwrap();
        points.push(vec![x, y]);
    }
    
    let solution = Solution;
    let result = solution.max_distance(side, &mut points, k);
    println!("{}", result);
}