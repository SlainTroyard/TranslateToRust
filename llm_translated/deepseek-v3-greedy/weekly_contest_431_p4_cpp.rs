use std::cmp::{min, Ordering};
use std::collections::VecDeque;
use std::io::{self, BufRead};

type LLArray = [i64; 5];

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let start: i32 = parts.next().unwrap().parse().unwrap();
        let end: i32 = parts.next().unwrap().parse().unwrap();
        let weight: i32 = parts.next().unwrap().parse().unwrap();
        intervals.push(vec![start, end, weight]);
    }
    let sol = Solution;
    let ans = sol.maximum_weight(&intervals);
    for val in ans {
        print!("{} ", val);
    }
    println!();
}

struct Solution;

impl Solution {
    pub fn maximum_weight(&self, intervals: &Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();
        let mut vec = Vec::with_capacity(2 * n + 1);
        vec.push((-1, -1));
        for (i, seg) in intervals.iter().enumerate() {
            vec.push((seg[0], -1));
            vec.push((seg[1], i as i32));
        }
        vec.sort_by(|a, b| a.0.cmp(&b.0));

        let n = vec.len();
        const INF: i64 = 1_000_000_000_000_000_000;
        let mut f = vec![vec![[INF; 5]; 5]; n];

        // Initialize f[0]
        for j in 1..=4 {
            f[0][j] = [INF; 5];
        }
        f[0][0] = [0, INF, INF, INF, INF];

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
                    let mid = (head + tail + 1) >> 1;
                    if vec[mid].0 < l {
                        head = mid;
                    } else {
                        tail = mid - 1;
                    }
                }

                for j in 1..=4 {
                    let mut tmp = f[head][j - 1];
                    tmp[0] -= intervals[idx as usize][2] as i64;
                    tmp[j] = idx as i64;
                    tmp.sort();
                    f[i][j] = min(f[i][j], tmp);
                }
            }
        }

        // Calculate the answer
        let mut ans = [INF; 5];
        for j in 1..=4 {
            ans = min(ans, f[n - 1][j]);
        }
        let mut ret = Vec::new();
        for j in 1..=4 {
            if ans[j] < INF {
                ret.push(ans[j] as i32);
            }
        }
        ret
    }
}