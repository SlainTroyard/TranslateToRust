use std::io::{self, BufRead};
use std::cmp::min;

const INF: i64 = 1_000_000_000_000_000_000;

struct Solution {}

impl Solution {
    pub fn maximumWeight(&self, intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n_intervals = intervals.len();
        let mut vec: Vec<(i32, i32)> = Vec::new();
        vec.push((-1, -1)); // Dummy entry

        for i in 0..n_intervals {
            let seg = &intervals[i];
            vec.push((seg[0], -1)); // Start point
            vec.push((seg[1], i as i32)); // End point with original index
        }

        vec.sort_unstable(); // Sort by the first element of the pair

        let n_vec = vec.len();
        type LLArray = [i64; 5];
        let mut f: Vec<[LLArray; 5]> = vec![[[0i64; 5]; 5]; n_vec];

        // Initialize f[0]
        for j in 1..=4 {
            f[0][j] = [INF; 5];
        }
        f[0][0] = [0, INF, INF, INF, INF];

        // Dynamic Programming
        for i in 1..n_vec {
            for j in 0..=4 {
                f[i][j] = f[i - 1][j]; // Copy from previous state
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
                    tmp[0] -= intervals[idx as usize][2] as i64; // Subtract weight
                    tmp[j] = idx as i64; // Store original index
                    tmp.sort_unstable(); // Sort the array
                    f[i][j] = min(f[i][j], tmp); // Take minimum
                }
            }
        }

        // Calculate Answer
        let mut ans: LLArray = [INF; 5];
        for j in 1..=4 {
            ans = min(ans, f[n_vec - 1][j]);
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().expect("Failed to read line").trim().parse().expect("Failed to parse n");
    let mut vec: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().expect("Failed to read line");
        let nums: Vec<i32> = line.trim().split_whitespace()
            .map(|s| s.parse().expect("Failed to parse int"))
            .collect();
        vec.push(nums);
    }

    let sol = Solution {};
    let ans = sol.maximumWeight(vec);

    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
    println!();
}