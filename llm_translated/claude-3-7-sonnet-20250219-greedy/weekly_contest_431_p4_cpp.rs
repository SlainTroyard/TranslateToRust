use std::cmp::min;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read intervals
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        vec.push(vec![parts[0], parts[1], parts[2]]);
    }
    
    let solution = Solution::new();
    let ans = solution.maximum_weight(vec);
    
    // Print the answer
    for (i, val) in ans.iter().enumerate() {
        print!("{}", val);
        if i < ans.len() - 1 {
            print!(" ");
        }
    }
    println!();
}

struct Solution;

impl Solution {
    fn new() -> Self {
        Solution
    }
    
    fn maximum_weight(&self, intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();

        // Create vector of (point, index) pairs
        let mut vec = Vec::new();
        vec.push((-1, -1));
        for i in 0..n {
            let seg = &intervals[i];
            vec.push((seg[0], -1));
            vec.push((seg[1], i as i32));
        }
        vec.sort();

        let n = vec.len();
        const INF: i64 = 1_000_000_000_000_000_000; // 1e18
        
        // Define the DP array
        type LLArray = [i64; 5];
        let mut f: Vec<[LLArray; 5]> = vec![[
            [0, INF, INF, INF, INF],
            [INF, INF, INF, INF, INF],
            [INF, INF, INF, INF, INF],
            [INF, INF, INF, INF, INF],
            [INF, INF, INF, INF, INF],
        ]; n];

        // Dynamic programming
        for i in 1..n {
            // Copy previous state
            for j in 0..=4 {
                f[i][j] = f[i-1][j];
            }
            
            let idx = vec[i].1;
            if idx >= 0 {
                let idx = idx as usize;
                let l = intervals[idx][0];
                
                // Binary search to find the rightmost position where vec[mid].first < l
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
                    let mut tmp = f[head][j-1];
                    tmp[0] -= intervals[idx][2] as i64;
                    tmp[j] = idx as i64;
                    tmp.sort();
                    
                    // Compare arrays lexicographically
                    if tmp < f[i][j] {
                        f[i][j] = tmp;
                    }
                }
            }
        }

        // Calculate the answer
        let mut ans = [INF, INF, INF, INF, INF];
        for j in 1..=4 {
            if f[n-1][j] < ans {
                ans = f[n-1][j];
            }
        }
        
        // Collect the result
        let mut ret = Vec::new();
        for j in 1..=4 {
            if ans[j] < INF {
                ret.push(ans[j] as i32);
            }
        }
        
        ret
    }
}