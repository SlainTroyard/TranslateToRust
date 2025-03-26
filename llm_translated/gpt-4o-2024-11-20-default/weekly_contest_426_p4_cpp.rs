use std::collections::VecDeque;
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
            let u = edge[0];
            let v = edge[1];
            graph[u].push(v);
            graph[v].push(u);
        }
    }

    fn bfs(&mut self, graph: &[Vec<usize>], color: &mut Vec<usize>, node_color: &mut Vec<usize>) {
        let n = graph.len();
        let mut queue = VecDeque::new();
        let mut visited = vec![false; n];

        queue.push_back((0, 0));

        while let Some((node, c)) = queue.pop_front() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            node_color[node] = c;
            color[c] += 1;

            for &adj in &graph[node] {
                if !visited[adj] {
                    queue.push_back((adj, (c + 1) % 2));
                }
            }
        }
    }

    fn max_target_nodes(&mut self, edges1: Vec<Vec<usize>>, edges2: Vec<Vec<usize>>) -> Vec<usize> {
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

        let mut result = vec![0; n];
        let max_color2 = usize::max(self.color2[0], self.color2[1]);

        for i in 0..n {
            result[i] = self.color1[self.node_color1[i]] + max_color2;
        }

        result
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input for edges1
    let n1: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid input for n1");
    let mut edges1 = Vec::new();
    for _ in 0..n1 - 1 {
        let edge: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid edge input for edges1"))
            .collect();
        edges1.push(edge);
    }

    // Read input for edges2
    let n2: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid input for n2");
    let mut edges2 = Vec::new();
    for _ in 0..n2 - 1 {
        let edge: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid edge input for edges2"))
            .collect();
        edges2.push(edge);
    }

    // Solve the problem
    let mut solution = Solution::new();
    let result = solution.max_target_nodes(edges1, edges2);

    // Print the result
    println!(
        "{}",
        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}