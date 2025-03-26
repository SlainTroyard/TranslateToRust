// Problem: Weekly Contest 426 Problem 3
use std::collections::VecDeque;
use std::io::{self, BufRead};

// Solution struct representing the logic
struct Solution;

impl Solution {
    /// Builds an adjacency list graph from the input edges
    fn build_graph(edges: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut graph = vec![Vec::new(); edges.len() + 1];
        for edge in edges {
            graph[edge[0]].push(edge[1]);
            graph[edge[1]].push(edge[0]);
        }
        graph
    }

    /// Performs a depth-first search to calculate reachable nodes
    fn dfs(
        graph: &Vec<Vec<usize>>,
        root: usize,
        parent: isize,
        k: isize,
        mut count: usize,
    ) -> usize {
        if k < 0 {
            return 0;
        }
        for &node in &graph[root] {
            if node as isize != parent {
                count += Self::dfs(graph, node, root as isize, k - 1, 1);
            }
        }
        count
    }

    /// Main logic for finding the maximum target nodes
    fn max_target_nodes(edges1: Vec<Vec<usize>>, edges2: Vec<Vec<usize>>, k: isize) -> Vec<usize> {
        let g1 = Self::build_graph(&edges1);
        let g2 = Self::build_graph(&edges2);

        let n = edges1.len() + 1; // Number of nodes in g1
        let m = edges2.len() + 1; // Number of nodes in g2

        let mut count = 0;
        let mut ans = Vec::new();
        for i in 0..m {
            count = count.max(Self::dfs(&g2, i, -1, k - 1, 1));
        }
        for i in 0..n {
            ans.push(count + Self::dfs(&g1, i, -1, k, 1));
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    // Read edges1 input
    let n1: usize = lines.next().unwrap().parse().unwrap();
    let mut edges1 = Vec::new();
    for _ in 0..n1 {
        let edge: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        edges1.push(edge);
    }

    // Read edges2 input
    let n2: usize = lines.next().unwrap().parse().unwrap();
    let mut edges2 = Vec::new();
    for _ in 0..n2 {
        let edge: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        edges2.push(edge);
    }

    // Read k input
    let k: isize = lines.next().unwrap().parse().unwrap();

    // Solve the problem
    let result = Solution::max_target_nodes(edges1, edges2, k);

    // Output the result
    for val in result {
        print!("{} ", val);
    }
    println!();
}