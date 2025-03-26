use std::collections::{VecDeque, HashSet};
use std::io::{self, BufRead};

struct Solution {
    tree1: Vec<Vec<usize>>,
    tree2: Vec<Vec<usize>>,
    color1: Vec<usize>,
    color2: Vec<usize>,
    node_color1: Vec<usize>,
    node_color2: Vec<usize>,
}

impl Solution {
    fn new() -> Self {
        Solution {
            tree1: Vec::new(),
            tree2: Vec::new(),
            color1: vec![0; 2],
            color2: vec![0; 2],
            node_color1: Vec::new(),
            node_color2: Vec::new(),
        }
    }

    fn build(edges: &[Vec<usize>], graph: &mut Vec<Vec<usize>>) {
        graph.resize(edges.len() + 1, Vec::new());
        for edge in edges {
            graph[edge[0]].push(edge[1]);
            graph[edge[1]].push(edge[0]);
        }
    }

    fn bfs(graph: &[Vec<usize>], color: &mut Vec<usize>, node_color: &mut Vec<usize>) {
        let n = graph.len();
        let mut queue = VecDeque::new();
        let mut visited = vec![false; n];
        queue.push_back((0, 0)); // Start BFS from node 0 with color 0

        while let Some((node, col)) = queue.pop_front() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            node_color[node] = col;
            color[col] += 1;

            for &adj in &graph[node] {
                if !visited[adj] {
                    queue.push_back((adj, (col + 1) % 2));
                }
            }
        }
    }

    fn max_target_nodes(&mut self, edges1: Vec<Vec<usize>>, edges2: Vec<Vec<usize>>) -> Vec<usize> {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;

        self.node_color1.resize(n, 0);
        self.node_color2.resize(m, 0);
        self.color1 = vec![0; 2];
        self.color2 = vec![0; 2];

        // Build trees
        Self::build(&edges1, &mut self.tree1);
        Self::build(&edges2, &mut self.tree2);

        // Color trees using BFS
        Self::bfs(&self.tree1, &mut self.color1, &mut self.node_color1);
        Self::bfs(&self.tree2, &mut self.color2, &mut self.node_color2);

        let mut result = vec![0; n];
        let max_color2 = self.color2[0].max(self.color2[1]);

        for i in 0..n {
            result[i] = self.color1[self.node_color1[i]] + max_color2;
        }

        result
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input for edges1
    let n1: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut edges1 = Vec::new();
    for _ in 0..n1 - 1 {
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
    let mut edges2 = Vec::new();
    for _ in 0..n2 - 1 {
        let edge: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        edges2.push(edge);
    }

    // Call the solution method
    let mut solution = Solution::new();
    let result = solution.max_target_nodes(edges1, edges2);

    // Output the result
    for val in result {
        print!("{} ", val);
    }
    println!();
}