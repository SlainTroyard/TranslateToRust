use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;
        
        // Build the graph
        let mut g: Vec<Vec<usize>> = vec![vec![]; n];
        for e in &invocations {
            g[e[0] as usize].push(e[1] as usize);
        }

        // Mark all suspicious methods
        let mut is_suspicious = vec![false; n];
        
        // DFS function to mark suspicious methods
        fn dfs(x: usize, g: &Vec<Vec<usize>>, is_suspicious: &mut Vec<bool>) {
            is_suspicious[x] = true;
            for &y in &g[x] {
                if !is_suspicious[y] { // Avoid infinite recursion
                    dfs(y, g, is_suspicious);
                }
            }
        }
        
        dfs(k, &g, &mut is_suspicious);

        // Check if there's an edge from [non-suspicious] -> [suspicious]
        for e in &invocations {
            let from = e[0] as usize;
            let to = e[1] as usize;
            if !is_suspicious[from] && is_suspicious[to] {
                // Cannot remove suspicious methods
                let mut ans: Vec<i32> = (0..n as i32).collect();
                return ans;
            }
        }

        // Remove all suspicious methods
        let mut ans = Vec::new();
        for i in 0..n {
            if !is_suspicious[i] {
                ans.push(i as i32);
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n, k, and invocationsSize
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let invocations_size: usize = iter.next().unwrap().parse().unwrap();
    
    // Read invocations
    let mut invocations = vec![vec![0; 2]; invocations_size];
    for i in 0..invocations_size {
        let line = lines.next().unwrap()?;
        let mut iter = line.split_whitespace();
        invocations[i][0] = iter.next().unwrap().parse().unwrap();
        invocations[i][1] = iter.next().unwrap().parse().unwrap();
    }
    
    // Solve the problem
    let s = Solution;
    let ans = s.remaining_methods(n, k, invocations);
    
    // Print the result
    for x in ans {
        print!("{} ", x);
    }
    println!();
    
    Ok(())
}