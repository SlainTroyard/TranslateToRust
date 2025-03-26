use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;
        
        // Build the graph
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
        for e in &invocations {
            g[e[0] as usize].push(e[1] as usize);
        }

        // Mark all suspicious methods
        let mut is_suspicious = vec![false; n];
        fn dfs(g: &Vec<Vec<usize>>, is_suspicious: &mut Vec<bool>, x: usize) {
            is_suspicious[x] = true;
            for &y in &g[x] {
                if !is_suspicious[y] { // Avoid infinite recursion
                    dfs(g, is_suspicious, y);
                }
            }
        }
        dfs(&g, &mut is_suspicious, k);

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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read input parameters
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let invocations_size: usize = iter.next().unwrap().parse().unwrap();
    
    // Read invocations
    let mut invocations = Vec::with_capacity(invocations_size);
    for _ in 0..invocations_size {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let from: i32 = iter.next().unwrap().parse().unwrap();
        let to: i32 = iter.next().unwrap().parse().unwrap();
        invocations.push(vec![from, to]);
    }
    
    // Solve and output
    let s = Solution{};
    let ans = s.remaining_methods(n, k, invocations);
    for x in ans {
        print!("{} ", x);
    }
    println!();
}