use std::collections::VecDeque;
use std::io::{self, BufRead};

struct Solution {
    tree1: Vec<Vec<i32>>,
    tree2: Vec<Vec<i32>>,
    color1: Vec<i32>,
    color2: Vec<i32>,
    node_color1: Vec<i32>,
    node_color2: Vec<i32>,
}

impl Solution {
    fn new() -> Self {
        Solution {
            tree1: Vec::new(),
            tree2: Vec::new(),
            color1: Vec::new(),
            color2: Vec::new(),
            node_color1: Vec::new(),
            node_color2: Vec::new(),
        }
    }

    fn build(&mut self, edges: &[Vec<i32>], mp: &mut Vec<Vec<i32>>) {
        mp.resize(edges.len() + 1, Vec::new());
        for e in edges {
            mp[e[0] as usize].push(e[1]);
            mp[e[1] as usize].push(e[0]);
        }
    }

    fn bfs(&mut self, mp: &[Vec<i32>], color: &mut [i32], node_color: &mut [i32]) {
        let n = mp.len();
        let mut q = VecDeque::new();
        let mut vis = vec![false; n];
        
        q.push_back((0, 0));
        
        while !q.is_empty() {
            let (i, c) = q.pop_front().unwrap();
            vis[i] = true;
            node_color[i] = c;
            color[c as usize] += 1;
            
            for &adj in &mp[i] {
                let adj = adj as usize;
                if !vis[adj] {
                    q.push_back((adj, (c + 1) % 2));
                }
            }
        }
    }

    fn max_target_nodes(&mut self, edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        
        self.node_color1 = vec![0; n];
        self.node_color2 = vec![0; m];
        self.color1 = vec![0; 2];
        self.color2 = vec![0; 2];
        
        // build trees
        self.build(&edges1, &mut self.tree1);
        self.build(&edges2, &mut self.tree2);
        
        // color trees using bfs
        self.bfs(&self.tree1, &mut self.color1, &mut self.node_color1);
        self.bfs(&self.tree2, &mut self.color2, &mut self.node_color2);
        
        let mut arr = vec![0; n];
        let mx = std::cmp::max(self.color2[0], self.color2[1]);
        
        for i in 0..n {
            arr[i] = self.color1[self.node_color1[i] as usize] + mx;
        }
        
        arr
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut solution = Solution::new();
    
    // Read n1
    let mut input = String::new();
    reader.read_line(&mut input)?;
    let n1: usize = input.trim().parse().expect("Invalid input for n1");
    
    // Read edges1
    let mut edges1 = Vec::with_capacity(n1 - 1);
    for _ in 0..n1 - 1 {
        input.clear();
        reader.read_line(&mut input)?;
        let values: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid edge input"))
            .collect();
        edges1.push(vec![values[0], values[1]]);
    }
    
    // Read n2
    input.clear();
    reader.read_line(&mut input)?;
    let n2: usize = input.trim().parse().expect("Invalid input for n2");
    
    // Read edges2
    let mut edges2 = Vec::with_capacity(n2 - 1);
    for _ in 0..n2 - 1 {
        input.clear();
        reader.read_line(&mut input)?;
        let values: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid edge input"))
            .collect();
        edges2.push(vec![values[0], values[1]]);
    }
    
    // Compute and output the result
    let result = solution.max_target_nodes(edges1, edges2);
    
    for val in result {
        print!("{} ", val);
    }
    println!();
    
    Ok(())
}