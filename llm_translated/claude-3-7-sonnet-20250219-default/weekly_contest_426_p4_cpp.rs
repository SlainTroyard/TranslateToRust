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
            tree1: vec![],
            tree2: vec![],
            color1: vec![],
            color2: vec![],
            node_color1: vec![],
            node_color2: vec![],
        }
    }

    fn build(&mut self, edges: &Vec<Vec<i32>>, mp: &mut Vec<Vec<i32>>) {
        *mp = vec![vec![]; edges.len() + 1];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            mp[u].push(e[1]);
            mp[v].push(e[0]);
        }
    }

    fn bfs(&self, mp: &Vec<Vec<i32>>, color: &mut Vec<i32>, node_color: &mut Vec<i32>) {
        let n = mp.len();
        let mut q = VecDeque::new();
        let mut vis = vec![false; n];
        q.push_back((0, 0)); // Start node with color 0

        while !q.is_empty() {
            let (i, c) = q.pop_front();
            let i_usize = i as usize;
            vis[i_usize] = true;
            node_color[i_usize] = c;
            color[c as usize] += 1;

            for &adj in &mp[i_usize] {
                let adj_usize = adj as usize;
                if !vis[adj_usize] {
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
        
        // Build trees
        self.build(&edges1, &mut self.tree1);
        self.build(&edges2, &mut self.tree2);
        
        // Color trees using BFS
        self.bfs(&self.tree1, &mut self.color1, &mut self.node_color1);
        self.bfs(&self.tree2, &mut self.color2, &mut self.node_color2);
        
        let mut arr = vec![0; n];
        let mx = self.color2[0].max(self.color2[1]);
        
        for i in 0..n {
            arr[i] = self.color1[self.node_color1[i] as usize] + mx;
        }
        
        arr
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input for edges1
    let n1: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut edges1 = vec![vec![0; 2]; n1 - 1];
    
    for i in 0..n1 - 1 {
        let line = lines.next().unwrap()?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        edges1[i][0] = nums[0];
        edges1[i][1] = nums[1];
    }
    
    // Input for edges2
    let n2: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut edges2 = vec![vec![0; 2]; n2 - 1];
    
    for i in 0..n2 - 1 {
        let line = lines.next().unwrap()?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        edges2[i][0] = nums[0];
        edges2[i][1] = nums[1];
    }
    
    // Call the solution method
    let mut solution = Solution::new();
    let result = solution.max_target_nodes(edges1, edges2);
    
    // Output the result
    for val in result {
        print!("{} ", val);
    }
    println!();
    
    Ok(())
}