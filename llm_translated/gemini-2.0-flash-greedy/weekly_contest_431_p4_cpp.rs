use std::cmp::{min, Ordering};
use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();

        let mut vec: Vec<(i32, i32)> = Vec::new();
        vec.push((-1, -1));
        for i in 0..n {
            let seg = &intervals[i];
            vec.push((seg[0], -1));
            vec.push((seg[1], i as i32));
        }
        vec.sort_by(|a, b| a.0.cmp(&b.0));

        let n = vec.len();
        const INF: i64 = 1_000_000_000_000_000_000;

        // Use type alias and Vec instead of VLA
        type LLArray = [i64; 5];
        let mut f: Vec<[LLArray; 5]> = vec![[[0; 5]; 5]; n];

        // Initialize f[0]
        for j in 1..=4 {
            f[0][j] = [INF, INF, INF, INF, INF];
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
                    f[i][j] = min(f[i][j], tmp); // std::array supports lexicographical comparison
                }
            }
        }

        // Calculate answer
        let mut ans: [i64; 5] = [INF, INF, INF, INF, INF];
        for j in 1..=4 {
            ans = min(ans, f[n - 1][j]);
        }
        let mut ret: Vec<i32> = Vec::new();
        for j in 1..=4 {
            if ans[j] < INF {
                ret.push(ans[j] as i32);
            }
        }
        ret
    }
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    let n: i32 = n_str.trim().parse().unwrap();

    let mut vec: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        vec.push(nums);
    }

    let sol = Solution {};
    let ans = sol.maximum_weight(vec);

    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
    println!();
}