use std::io;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn construct_grid_layout(n: usize, edges: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
        // Step 1: Build adjacency list
        let mut graph = vec![Vec::new(); n];
        for &(x, y) in &edges {
            graph[x].push(y);
            graph[y].push(x);
        }

        // Step 2: Find the node with the smallest degree to start
        let mut x = 0;
        for i in 0..graph.len() {
            if graph[i].len() < graph[x].len() {
                x = i;
            }
        }

        // Step 3: Construct the first row
        let mut row = vec![x];
        let mut visited = vec![false; n];
        visited[x] = true;
        let deg_st = graph[x].len();

        loop {
            let mut next = None;
            for &y in &graph[x] {
                if !visited[y] && (next.is_none() || graph[y].len() < graph[next.unwrap()].len()) {
                    next = Some(y);
                }
            }

            match next {
                Some(nxt) => {
                    x = nxt;
                    row.push(x);
                    visited[x] = true;
                }
                None => break,
            }

            if graph[x].len() <= deg_st {
                break;
            }
        }

        // Step 4: Construct the remaining rows
        let mut ans = vec![Vec::new(); n / row.len()];
        ans[0] = row;

        for i in 1..ans.len() {
            for &x in &ans[i - 1] {
                for &y in &graph[x] {
                    if !visited[y] {
                        visited[y] = true;
                        ans[i].push(y);
                        break;
                    }
                }
            }
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n_edges: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let n = n_edges[0];
    let edges_size = n_edges[1];

    let mut edges = Vec::new();
    for _ in 0..edges_size {
        let mut edge_input = String::new();
        io::stdin().read_line(&mut edge_input).unwrap();
        let edge: Vec<usize> = edge_input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        edges.push((edge[0], edge[1]));
    }

    // Solve the problem
    let solution = Solution;
    let res = solution.construct_grid_layout(n, edges);

    // Print the result in the required format
    for row in res {
        println!("{}", row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}