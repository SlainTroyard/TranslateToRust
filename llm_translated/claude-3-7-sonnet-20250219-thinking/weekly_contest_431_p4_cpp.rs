use std::io::{self, BufRead};
use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();

        // Create a vector of pairs (time point, index)
        // Index -1 means it's a start point, otherwise it's an end point
        let mut vec = vec![(-1, -1)];
        for i in 0..n {
            let seg = &intervals[i];
            vec.push((seg[0], -1));
            vec.push((seg[1], i as i32));
        }
        vec.sort();

        let n = vec.len();
        const INF: i64 = 1_000_000_000_000_000_000; // 1e18
        
        // Define our 3D dp array structure
        type LLArray = [i64; 5];
        let mut f = vec![[LLArray::default(); 5]; n];

        // Initialize f[0]
        for j in 1..=4 {
            f[0][j] = [INF, INF, INF, INF, INF];
        }
        f[0][0] = [0, INF, INF, INF, INF];

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
                
                // Binary search to find the rightmost point before the start of current interval
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
                    tmp[0] -= intervals[idx][2] as i64; // Subtract weight (negative to minimize)
                    tmp[j] = idx as i64;
                    tmp.sort(); // Sort to ensure lexicographical comparison works
                    
                    // Compare and update if better
                    if tmp < f[i][j] {
                        f[i][j] = tmp;
                    }
                }
            }
        }

        // Find the best answer among all possible choices (1 to 4 intervals)
        let mut ans = [INF, INF, INF, INF, INF];
        for j in 1..=4 {
            if f[n-1][j] < ans {
                ans = f[n-1][j];
            }
        }

        // Collect the indices of the chosen intervals
        let mut ret = Vec::new();
        for j in 1..=4 {
            if ans[j] < INF {
                ret.push(ans[j] as i32);
            }
        }
        
        ret
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse the number of intervals
    let n = lines.next().unwrap()?.parse::<usize>().unwrap();
    let mut vec = vec![vec![0; 3]; n];
    
    // Parse interval data: start, end, weight
    for i in 0..n {
        let line = lines.next().unwrap()?;
        let values: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        
        vec[i][0] = values[0]; // start
        vec[i][1] = values[1]; // end
        vec[i][2] = values[2]; // weight
    }
    
    // Solve the problem
    let ans = Solution::maximum_weight(vec);
    
    // Print the result
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
    println!();
    
    Ok(())
}