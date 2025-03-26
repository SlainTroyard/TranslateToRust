use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn max_target_nodes(
        &mut self,
        edges1: Vec<Vec<i32>>,
        edges2: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;

        let mut tree1: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut tree2: Vec<Vec<usize>> = vec![Vec::new(); m];

        let mut color1: Vec<i32> = vec![0; 2];
        let mut color2: Vec<i32> = vec![0; 2];

        let mut node_color1: Vec<i32> = vec![0; n];
        let mut node_color2: Vec<i32> = vec![0; m];

        // build trees
        self.build(&edges1, &mut tree1);
        self.build(&edges2, &mut tree2);

        // color trees using bfs
        self.bfs(&tree1, &mut color1, &mut node_color1);
        self.bfs(&tree2, &mut color2, &mut node_color2);

        let mut arr: Vec<i32> = vec![0; n];
        let mx = color2[0].max(color2[1]);
        for i in 0..n {
            arr[i] = color1[node_color1[i] as usize] + mx;
        }
        arr
    }

    fn build(&self, edges: &Vec<Vec<i32>>, mp: &mut Vec<Vec<usize>>) {
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            mp[u].push(v);
            mp[v].push(u);
        }
    }

    fn bfs(
        &self,
        mp: &Vec<Vec<usize>>,
        color: &mut Vec<i32>,
        node_color: &mut Vec<i32>,
    ) {
        let n = mp.len();
        let mut q: VecDeque<(usize, i32)> = VecDeque::new();
        let mut vis: Vec<bool> = vec![false; n];
        q.push_back((0, 0));

        while !q.is_empty() {
            if let Some((i, c)) = q.pop_front() {
                vis[i] = true;
                node_color[i] = c;
                color[c as usize] += 1;

                for &adj in &mp[i] {
                    if !vis[adj] {
                        q.push_back((adj, (c + 1) % 2));
                    }
                }
            }
        }
    }
}

fn main() {
    let mut solution = Solution {};

    // Input for edges1
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n1: i32 = input.trim().parse().unwrap();

    let mut edges1: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n1 - 1 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges1.push(nums);
    }

    // Input for edges2
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n2: i32 = input.trim().parse().unwrap();

    let mut edges2: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n2 - 1 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges2.push(nums);
    }

    // Call the solution method
    let result = solution.max_target_nodes(edges1, edges2);

    // Output the result
    for val in result {
        print!("{} ", val);
    }
    println!();
}