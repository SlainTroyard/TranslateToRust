// Problem: Weekly Contest 426 Problem 3
use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    fn build_graph(edges: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        // Create a graph with one extra node than the max value in edges
        let mut g = vec![Vec::new(); edges.len() + 1];
        for edge in edges {
            g[edge[0]].push(edge[1]);
            g[edge[1]].push(edge[0]);
        }
        g
    }

    fn dfs(graph: &Vec<Vec<usize>>, root: usize, parent: isize, k: isize, count: usize) -> usize {
        if k < 0 {
            return 0;
        }
        let mut total_count = count;
        for &node in &graph[root] {
            if node as isize != parent {
                total_count += Solution::dfs(graph, node, root as isize, k - 1, count);
            }
        }
        total_count
    }

    fn max_target_nodes(edges1: Vec<Vec<usize>>, edges2: Vec<Vec<usize>>, k: usize) -> Vec<usize> {
        let graph1 = Solution::build_graph(edges1);
        let graph2 = Solution::build_graph(edges2);
        let mut count = 0;
        let n = graph1.len();
        let m = graph2.len();
        let mut result = Vec::new();

        for i in 0..m {
            count = max(count, Solution::dfs(&graph2, i, -1, k as isize - 1, 1));
        }
        for i in 0..n {
            result.push(count + Solution::dfs(&graph1, i, -1, k as isize, 1));
        }
        result
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input for edges1
    let n1: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut edges1 = Vec::with_capacity(n1);
    for _ in 0..n1 {
        let edge: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        edges1.push(edge);
    }

    // Input for edges2
    let n2: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut edges2 = Vec::with_capacity(n2);
    for _ in 0..n2 {
        let edge: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        edges2.push(edge);
    }

    // Input for k
    let k: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Call the solution method
    let solution = Solution;
    let result = solution.max_target_nodes(edges1, edges2, k);

    // Output the result
    for val in result {
        print!("{val} ");
    }
    println!();
}