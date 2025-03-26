use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();

    // Read the number of intervals
    stdin_lock.read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read the intervals
    let mut intervals: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).unwrap();
        let interval: Vec<i32> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        intervals.push(interval);
    }

    // Solve the problem
    let solution = Solution::maximum_weight(intervals);

    // Output the result
    for (i, &val) in solution.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
}

struct Solution;

impl Solution {
    fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();
        let mut vec: Vec<(i32, isize)> = Vec::with_capacity(2 * n + 1);
        vec.push((-1, -1));
        for (i, seg) in intervals.iter().enumerate() {
            vec.push((seg[0], -1));
            vec.push((seg[1], i as isize));
        }
        vec.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let n = vec.len();
        const INF: i64 = 1e18 as i64;
        let mut f: Vec<[i64; 5]> = vec![[INF; 5]; n];

        // Initialize f[0]
        for j in 1..=4 {
            f[0][j] = INF;
        }
        f[0][0] = 0;

        // Dynamic programming
        for i in 1..n {
            for j in 0..=4 {
                f[i][j] = f[i - 1][j];
            }
            let idx = vec[i].1;
            if idx >= 0 {
                let l = intervals[idx as usize][0];
                let mut head = 0;
                let mut tail = i - 1;
                while head < tail {
                    let mid = (head + tail + 1) / 2;
                    if vec[mid].0 < l {
                        head = mid;
                    } else {
                        tail = mid - 1;
                    }
                }

                for j in 1..=4 {
                    let mut tmp = f[head][j - 1];
                    tmp[0] -= intervals[idx as usize][2] as i64;
                    tmp[j] = idx;
                    tmp.sort_unstable();
                    if tmp.cmp(&f[i][j]) == Ordering::Less {
                        f[i][j] = tmp;
                    }
                }
            }
        }

        // Calculate the answer
        let mut ans: [i64; 5] = [INF; 5];
        for j in 1..=4 {
            if ans.cmp(&f[n - 1][j]) == Ordering::Greater {
                ans = f[n - 1][j];
            }
        }

        ans.iter()
            .filter(|&&x| x < INF)
            .map(|&x| x as i32)
            .collect()
    }
}