use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    fn build(edges: &Vec<Vec<i32>>, mp: &mut Vec<Vec<i32>>) {
        for e in edges {
            mp[e[0] as usize].push(e[1]);
            mp[e[1] as usize].push(e[0]);
        }
    }

    fn bfs(mp: &Vec<Vec<i32>>, color: &mut Vec<i32>, node_color: &mut Vec<i32>) {
        let n = mp.len();
        let mut q: VecDeque<(usize, i32)> = VecDeque::new();
        let mut vis = vec![false; n];
        q.push_back((0, 0));

        while !q.is_empty() {
            if let Some((i, c)) = q.pop_front() {
                vis[i] = true;
                node_color[i] = c;
                color[c as usize] += 1;

                for &adj in &mp[i] {
                    if !vis[adj as usize] {
                        q.push_back((adj as usize, (c + 1) % 2));
                    }
                }
            }
        }
    }

    fn max_target_nodes(
        &self,
        edges1: &Vec<Vec<i32>>,
        edges2: &Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;

        let mut tree1: Vec<Vec<i32>> = vec![Vec::new(); n];
        let mut tree2: Vec<Vec<i32>> = vec![Vec::new(); m];

        let mut color1 = vec![0; 2];
        let mut color2 = vec![0; 2];

        let mut node_color1 = vec![0; n];
        let mut node_color2 = vec![0; m];

        // build trees
        Solution::build(edges1, &mut tree1);
        Solution::build(edges2, &mut tree2);

        // color trees using bfs
        Solution::bfs(&tree1, &mut color1, &mut node_color1);
        Solution::bfs(&tree2, &mut color2, &mut node_color2);

        let mut arr = vec![0; n];
        let mx = std::cmp::max(color2[0], color2[1]);

        for i in 0..n {
            arr[i] = color1[node_color1[i] as usize] + mx;
        }

        arr
    }
}

fn main() -> io::Result<()> {
    let solution = Solution {};

    // Input for edges1
    let mut n1_str = String::new();
    io::stdin().read_line(&mut n1_str)?;
    let n1: i32 = n1_str.trim().parse().unwrap();

    let mut edges1: Vec<Vec<i32>> = Vec::new();
    for _ in 0..(n1 - 1) {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let nums: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges1.push(nums);
    }

    // Input for edges2
    let mut n2_str = String::new();
    io::stdin().read_line(&mut n2_str)?;
    let n2: i32 = n2_str.trim().parse().unwrap();

    let mut edges2: Vec<Vec<i32>> = Vec::new();
    for _ in 0..(n2 - 1) {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let nums: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges2.push(nums);
    }

    // Call the solution method
    let result = solution.max_target_nodes(&edges1, &edges2);

    // Output the result
    for val in result {
        print!("{} ", val);
    }
    println!();

    Ok(())
}