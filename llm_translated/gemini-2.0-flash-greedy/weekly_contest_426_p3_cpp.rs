use std::io;
use std::io::prelude::*;
use std::cmp::max;

struct Solution {}

impl Solution {
    fn build_graph(edges: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); edges.len() + 1];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        g
    }

    fn dfs(g: &Vec<Vec<usize>>, root: usize, par: i32, k: i32) -> i32 {
        if k < 0 {
            return 0;
        }

        let mut count = 1;
        for &node in &g[root] {
            if node as i32 != par {
                count += Self::dfs(g, node, root as i32, k - 1);
            }
        }
        count
    }

    fn max_target_nodes(edges1: &Vec<Vec<i32>>, edges2: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let g1 = Self::build_graph(edges1);
        let g2 = Self::build_graph(edges2);
        let mut count = 0;
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        let mut ans: Vec<i32> = Vec::new();

        for i in 0..m {
            count = max(count, Self::dfs(&g2, i, -1, k - 1));
        }

        for i in 0..n {
            ans.push(count + Self::dfs(&g1, i, -1, k));
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // Input for edges1
    let n1: usize = iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let mut edges1: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n1 {
        let line = iterator.next().unwrap().unwrap();
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges1.push(values);
    }

    // Input for edges2
    let n2: usize = iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let mut edges2: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n2 {
        let line = iterator.next().unwrap().unwrap();
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges2.push(values);
    }

    // Input for k
    let k: i32 = iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    // Call the solution method
    let result = Solution {}.max_target_nodes(&edges1, &edges2, k);

    // Output the result
    for val in result {
        print!("{} ", val);
    }
    println!();
}