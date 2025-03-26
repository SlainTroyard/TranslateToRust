use std::io;

struct Solution {}

impl Solution {
    fn max_distance(side: i32, mut points: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = points.len();
        let side = side as i64;

        // Custom sorting based on perimeter order
        points.sort_by(|a, b| {
            let ord = |x: i32, y: i32| -> i64 {
                let s = side;
                if y == 0 {
                    x as i64
                } else if x == s as i32 {
                    s + y as i64
                } else if y == s as i32 {
                    s * 3 - x as i64
                } else {
                    s * 4 - y as i64
                }
            };
            let a_val = ord(a[0], a[1]);
            let b_val = ord(b[0], b[1]);
            a_val.cmp(&b_val)
        });

        // Distance function
        let dis = |i: usize, j: usize| -> i32 {
            let dx = points[i][0] - points[j][0];
            let dy = points[i][1] - points[j][1];
            dx.abs() + dy.abs()
        };

        // Check function
        let check = |lim: i32| -> bool {
            let mut vec = vec![0];
            for i in 1..n {
                if vec.len() >= k as usize {
                    break;
                }
                if dis(i, *vec.last().unwrap()) >= lim {
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
                let mut vec = vec![i];
                for j in 1..k as usize {
                    let mut pos = vec[j - 1] + 1;
                    while pos < 2 * n && dis(pos % n, vec[j - 1] % n) < lim {
                        pos += 1;
                    }
                    if pos >= 2 * n {
                        return false;
                    }
                    vec.push(pos);
                }
                if vec.last().unwrap() < &i + n && dis(i, vec.last().unwrap() % n) >= lim {
                    return true;
                }
            }
            false
        };

        // Binary search
        let mut head = 1;
        let mut tail = side as i32;
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
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let side: i32 = parts.next().unwrap().parse().unwrap();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut coords = line.trim().split_whitespace();
        let x: i32 = coords.next().unwrap().parse().unwrap();
        let y: i32 = coords.next().unwrap().parse().unwrap();
        points.push(vec![x, y]);
    }

    let sol = Solution {};
    println!("{}", sol.max_distance(side, points, k));
}