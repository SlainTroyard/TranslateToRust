use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_distance(side: i32, points: &Vec<Vec<i32>>, k: i32) -> i32 {
        let n = points.len();

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

        let mut points = points.clone();
        points.sort_by_key(|p| ord(p[0] as i64, p[1] as i64));

        let dis = |i: usize, j: usize| -> i32 {
            (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs()
        };

        let check = |lim: i32| -> bool {
            let mut vec = vec![0];
            for i in 1..n {
                if vec.len() == k as usize {
                    break;
                }
                if dis(i, *vec.last().unwrap()) >= lim {
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
                if *vec.last().unwrap() >= n * 2 {
                    break;
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
                if *vec.last().unwrap() < i + n && dis(i, *vec.last().unwrap() % n) >= lim {
                    return true;
                }
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let first_line = lines.next().unwrap()?;
    let mut values = first_line.split_whitespace();
    let side: i32 = values.next().unwrap().parse().unwrap();
    let n: usize = values.next().unwrap().parse().unwrap();
    let k: i32 = values.next().unwrap().parse().unwrap();

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let mut coords = line.split_whitespace();
        let x: i32 = coords.next().unwrap().parse().unwrap();
        let y: i32 = coords.next().unwrap().parse().unwrap();
        points.push(vec![x, y]);
    }

    // Calculate and print result
    let solution = Solution::max_distance(side, &points, k);
    println!("{}", solution);

    Ok(())
}