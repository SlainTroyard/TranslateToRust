use std::io::{self, BufRead};
use std::cmp;

struct Solution;

impl Solution {
    fn build_graph(edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut g = vec![Vec::new(); edges.len() + 1];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        return g;
    }

    fn dfs(g: &Vec<Vec<i32>>, root: i32, par: i32, k: i32, count: i32) -> i32 {
        if k < 0 {
            return 0;
        }
        
        let mut total_count = count;
        for &node in &g[root as usize] {
            if node != par {
                total_count += Self::dfs(g, node, root, k - 1, 0);
            }
        }
        return total_count;
    }

    fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let g1 = Self::build_graph(&edges1);
        let g2 = Self::build_graph(&edges2);
        
        let mut count = 0;
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        let mut ans = Vec::new();
        
        for i in 0..m {
            count = cmp::max(count, Self::dfs(&g2, i as i32, -1, k - 1, 1));
        }
        
        for i in 0..n {
            ans.push(count + Self::dfs(&g1, i as i32, -1, k, 1));
        }
        
        return ans;
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input for edges1
    let n1: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut edges1 = vec![vec![0; 2]; n1];
    for i in 0..n1 {
        let line = lines.next().unwrap()?;
        let values: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges1[i][0] = values[0];
        edges1[i][1] = values[1];
    }
    
    // Input for edges2
    let n2: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut edges2 = vec![vec![0; 2]; n2];
    for i in 0..n2 {
        let line = lines.next().unwrap()?;
        let values: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges2[i][0] = values[0];
        edges2[i][1] = values[1];
    }
    
    // Input for k
    let k: i32 = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Call the solution method
    let result = Solution::max_target_nodes(edges1, edges2, k);
    
    // Output the result
    for val in result {
        print!("{} ", val);
    }
    println!();
    
    Ok(())
}