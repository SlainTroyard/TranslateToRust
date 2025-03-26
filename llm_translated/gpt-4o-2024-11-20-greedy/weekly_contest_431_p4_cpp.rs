use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::cmp::min;

struct Solution;

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i64>>) -> Vec<i64> {
        let n = intervals.len();

        type Pii = (i64, i64);
        let mut vec: Vec<Pii> = vec![(-1, -1)];
        for (i, seg) in intervals.iter().enumerate() {
            vec.push((seg[0], -1));
            vec.push((seg[1], i as i64));
        }
        vec.sort();

        let n = vec.len();
        const INF: i64 = 1e18 as i64;
        type LLArray = [i64; 5];
        let mut f: Vec<[LLArray; 5]> = vec![[[INF; 5]; 5]; n];

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
                let idx = idx as usize;
                let l = intervals[idx][0];
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
                    tmp[0] -= intervals[idx][2];
                    tmp[j] = idx as i64;
                    tmp.sort();
                    f[i][j] = min(f[i][j], tmp);
                }
            }
        }

        // Calculate the answer
        let mut ans: LLArray = [INF; 5];
        for j in 1..=4 {
            ans = min(ans, f[n - 1][j]);
        }
        let mut ret = Vec::new();
        for j in 1..=4 {
            if ans[j] < INF {
                ret.push(ans[j]);
            }
        }
        ret
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of intervals
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the intervals
    let mut intervals = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        intervals.push(parts);
    }

    // Solve the problem
    let solution = Solution;
    let ans = solution.maximum_weight(intervals);

    // Print the result
    for (i, val) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
}