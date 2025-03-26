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
    
    let sol = Solution::new();
    let ans = sol.maximum_weight(vec);
    
    // Output the result
    for (i, val) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
}

struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }
    
    pub fn maximum_weight(&self, intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();

        // Create a vector of events (start or end points)
        let mut vec = Vec::new();
        vec.push((-1, -1)); // Dummy event at the beginning
        for i in 0..n {
            let seg = &intervals[i];
            vec.push((seg[0], -1)); // Start point
            vec.push((seg[1], i as i32)); // End point with index
        }
        vec.sort();

        let event_count = vec.len();
        const INF: i64 = 1_000_000_000_000_000_000; // 1e18
        
        // Dynamic programming table: f[i][j] represents best state after considering first i events
        // and picking j segments.
        // Each state is an array of 5 i64s where the first element is the negative sum of weights
        // and the rest contains the indices of chosen segments.
        let mut f = vec![[[INF; 5]; 5]; event_count];
        
        // Initialize starting state
        f[0][0][0] = 0;
        for j in 1..=4 {
            for k in 0..5 {
                f[0][j][k] = INF;
            }
        }

        // Dynamic programming
        for i in 1..event_count {
            // Copy previous state
            for j in 0..=4 {
                f[i][j] = f[i-1][j];
            }
            
            let idx = vec[i].1;
            if idx >= 0 {
                let idx = idx as usize;
                let l = intervals[idx][0];
                
                // Binary search to find the latest event before this segment starts
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
                    
                    // Check if tmp is better than current state
                    if tmp < f[i][j] {
                        f[i][j] = tmp;
                    }
                }
            }
        }

        // Find the best solution
        let mut ans = [INF; 5];
        for j in 1..=4 {
            if f[event_count-1][j] < ans {
                ans = f[event_count-1][j];
            }
        }
        
        // Convert to result format
        let mut ret = Vec::new();
        for j in 1..=4 {
            if ans[j] < INF {
                ret.push(ans[j] as i32);
            }
        }
        
        ret
    }
}