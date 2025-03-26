use std::io;

pub struct Solution;

impl Solution {
    pub fn max_distance(&self, side: i32, points: &mut Vec<Vec<i32>>, K: i32) -> i32 {
        let n = points.len();
        let ord = |x: i32, y: i32| -> i64 {
            let s = side as i64;
            if y == 0 {
                x as i64
            } else if x == side {
                s + y as i64
            } else if y == side {
                s * 3 - x as i64
            } else {
                s * 4 - y as i64
            }
        };
        points.sort_by(|a, b| {
            let a_ord = ord(a[0], a[1]);
            let b_ord = ord(b[0], b[1]);
            a_ord.cmp(&b_ord)
        });

        let dis = |i: usize, j: usize| -> i32 {
            let dx = points[i][0] - points[j][0];
            let dy = points[i][1] - points[j][1];
            dx.abs() + dy.abs()
        };

        let check = |lim: i32| -> bool {
            let n = points.len() as i32;
            let k = K as usize;
            let mut vec = vec![0];
            for i in 1..n as usize {
                if vec.len() >= k {
                    break;
                }
                if dis(i, *vec.last().unwrap()) >= lim {
                    vec.push(i);
                }
            }
            if vec.len() < k {
                return false;
            }
            if dis(vec[0], vec[vec.len() - 1]) >= lim {
                return true;
            }
            for i in 1..n as usize {
                vec[0] = i;
                for j in 1..k {
                    while dis(
                        (vec[j] % n as usize),
                        (vec[j - 1] % n as usize),
                    ) < lim
                    {
                        vec[j] += 1;
                        if vec[j] >= 2 * n as usize {
                            return false;
                        }
                    }
                }
                if vec.last().unwrap() < &(i + n as usize)
                    && dis(
                        i,
                        vec.last().unwrap() % n as usize,
                    ) >= lim
                {
                    return true;
                }
            }
            false
        };

        let (mut head, mut tail) = (1, side);
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let side: i32 = parts.next().unwrap().parse().unwrap();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut coords = line.split_whitespace();
        let x: i32 = coords.next().unwrap().parse().unwrap();
        let y: i32 = coords.next().unwrap().parse().unwrap();
        points.push(vec![x, y]);
    }

    let solution = Solution {};
    let result = solution.max_distance(side, &mut points, k);
    println!("{}", result);
}