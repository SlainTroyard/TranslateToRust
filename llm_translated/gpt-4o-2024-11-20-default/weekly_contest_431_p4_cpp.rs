use std::cmp::{min, Ordering};
use std::collections::VecDeque;
use std::io::{self, BufRead};

// Struct to encapsulate the Solution logic
struct Solution;

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();
        let inf: i64 = 1e18 as i64;

        type LLArray = [i64; 5]; // Array with 5 elements for type alias
        let mut vec = Vec::new();
        vec.push((-1, -1));

        // Processing the input intervals into `vec`
        for (i, seg) in intervals.iter().enumerate() {
            vec.push((seg[0], -1));
            vec.push((seg[1], i as i32));
        }
        vec.sort();

        let n = vec.len();
        let mut f = vec![vec![[inf; 5]; 5]; n];

        // Initializing f[0]
        for j in 1..=4 {
            f[0][j] = [inf; 5];
        }
        f[0][0] = [0, inf, inf, inf, inf];

        // Dynamic Programming Implementation
        for i in 1..n {
            for j in 0..=4 {
                f[i][j] = f[i - 1][j];
            }
            let idx = vec[i].1;
            if idx >= 0 {
                let idx = idx as usize;
                let l = intervals[idx][0];
                
                // Binary search to find the largest index with vec[mid].0 < l
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
                    tmp[0] -= intervals[idx][2] as i64;
                    tmp[j] = idx as i64;
                    tmp.sort();
                    f[i][j] = f[i][j].iter().zip(&tmp).map(|(a, b)| min(*a, *b)).collect::<Vec<i64>>().try_into().unwrap_or([inf; 5]);
                }
            }
        }

        // Computing the final answer
        let mut ans: LLArray = [inf; 5];
        for j in 1..=4 {
            ans = ans.iter().zip(&f[n - 1][j]).map(|(a, b)| min(*a, *b)).collect::<Vec<i64>>().try_into().unwrap_or([inf; 5]);
        }
        let mut ret = Vec::new();
        for &x in &ans[1..=4] {
            if x < inf {
                ret.push(x as i32);
            }
        }
        ret
    }
}

fn main() {
    // Reading input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // First line: n (number of intervals)
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Next n lines: each line contains 3 integers (start, end, weight)
    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        intervals.push(nums);
    }

    // Invoking the solution
    let sol = Solution::maximum_weight(intervals);
    
    // Printing the result
    let output: String = sol.iter().map(ToString::to_string).collect::<Vec<_>>().join(" ");
    println!("{}", output);
}