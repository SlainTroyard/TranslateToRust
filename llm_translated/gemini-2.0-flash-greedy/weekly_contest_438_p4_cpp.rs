use std::cmp::max;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut first_split = first_line.split_whitespace();
    let side: i32 = first_split.next().unwrap().parse().unwrap();
    let n: usize = first_split.next().unwrap().parse().unwrap();
    let k: usize = first_split.next().unwrap().parse().unwrap();

    let mut points: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut split = line.split_whitespace();
        let x: i32 = split.next().unwrap().parse().unwrap();
        let y: i32 = split.next().unwrap().parse().unwrap();
        points.push(vec![x, y]);
    }

    let sol = Solution {};
    println!("{}", sol.max_distance(side, points, k));
}

struct Solution {}

impl Solution {
    fn max_distance(&self, side: i32, points: Vec<Vec<i32>>, k: usize) -> i32 {
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

        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_by_key(|&i| ord(points[i][0] as i64, points[i][1] as i64));

        let dis = |i: usize, j: usize| -> i32 {
            (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs()
        };

        let check = |lim: i32| -> bool {
            let mut vec: Vec<usize> = vec![indices[0]];
            let mut idx = 1;
            while idx < n && vec.len() < k {
                if dis(indices[idx], *vec.last().unwrap()) >= lim {
                    vec.push(indices[idx]);
                }
                idx += 1;
            }
            if vec.len() < k {
                return false;
            }
            if dis(vec[0], *vec.last().unwrap()) >= lim {
                return true;
            }

            let mut vec_extended: Vec<usize> = vec![0; k];
            for i in 1..n {
                vec_extended[0] = indices[i];
                for j in 1..k {
                    let mut current_index = vec_extended[j - 1];
                    loop {
                        current_index += 1;
                        if current_index >= n * 2 {
                            return false;
                        }
                        vec_extended[j] = indices[current_index % n];
                        if dis(vec_extended[j], vec_extended[j - 1]) >= lim {
                            break;
                        }
                    }
                }

                if vec_extended.last().copied().unwrap() < indices[i] + n as usize
                    && dis(indices[i], vec_extended.last().copied().unwrap() % n) >= lim
                {
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