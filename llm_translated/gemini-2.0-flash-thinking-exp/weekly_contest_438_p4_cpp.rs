use std::io;
use std::vec::Vec;
use std::cmp::max;

fn solve() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.trim().split_whitespace();
    let side: i32 = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    let mut points: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let mut iter = input.trim().split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        points.push(vec![x, y]);
    }

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

    points.sort_by(|a, b| ord(a[0] as i64, a[1] as i64).cmp(&ord(b[0] as i64, b[1] as i64)));

    let dis = |points: &Vec<Vec<i32>>, i: usize, j: usize| -> i32 {
        (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs()
    };

    let check = |lim: i32, points: &Vec<Vec<i32>>, k: usize| -> bool {
        let n = points.len();
        let mut vec: Vec<usize> = vec![0];
        for i in 1..n {
            if vec.len() < k && dis(points, i, *vec.last().unwrap()) >= lim {
                vec.push(i);
            }
        }
        if vec.len() < k {
            return false;
        }
        if dis(points, vec[0], *vec.last().unwrap()) >= lim {
            return true;
        }
        let mut vec_copy = vec.clone(); // Create a mutable copy to avoid modifying original vec in loop
        for i in 1..n {
            vec_copy[0] = i;
            for j in 1..k {
                while dis(points, vec_copy[j] % n, vec_copy[j - 1] % n) < lim {
                    vec_copy[j] += 1;
                    if vec_copy[j] >= n * 2 {
                        return false;
                    }
                }
            }
            if vec_copy.last().unwrap() < &(i + n) && dis(points, i, vec_copy.last().unwrap() % n) >= lim {
                return true;
            }
        }
        false
    };

    let mut head = 1;
    let mut tail = side;
    let mut ans = 0;
    while head <= tail {
        let mid = head + (tail - head) / 2;
        if check(mid, &points, k) {
            ans = mid;
            head = mid + 1;
        } else {
            tail = mid - 1;
        }
    }

    println!("{}", ans);
    Ok(())
}

fn main() -> io::Result<()> {
    solve()
}