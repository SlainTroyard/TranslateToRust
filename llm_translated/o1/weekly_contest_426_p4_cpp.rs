// Translation of the C++ solution for LeetCode Weekly Contest 426 Problem 4 into Rust.
// The logic is preserved exactly while using idiomatic Rust and proper error handling.

use std::cmp;
use std::collections::VecDeque;
use std::io::{self, BufRead};

// A struct to hold all data needed for the solution
struct Solution {
    // Adjacency lists for both trees
    tree1: Vec<Vec<usize>>,
    tree2: Vec<Vec<usize>>,
    // colorX tracks how many nodes of each color (0 or 1) exist in a tree
    color1: Vec<usize>,
    color2: Vec<usize>,
    // nodeColorX stores the color of each node
    node_color1: Vec<usize>,
    node_color2: Vec<usize>,
}

impl Solution {
    // Create an empty Solution
    fn new() -> Self {
        Self {
            tree1: Vec::new(),
            tree2: Vec::new(),
            color1: vec![0; 2],
            color2: vec![0; 2],
            node_color1: Vec::new(),
            node_color2: Vec::new(),
        }
    }

    // Build the adjacency list from the given edge list
    fn build(&mut self, edges: &Vec<Vec<usize>>, mp: &mut Vec<Vec<usize>>) {
        // Each tree has (edges.len() + 1) nodes
        mp.resize(edges.len() + 1, Vec::new());
        for e in edges {
            let u = e[0];
            let v = e[1];
            mp[u].push(v);
            mp[v].push(u);
        }
    }

    // Use BFS to color the tree (0-based) starting from node 0
    // color[c] counts how many nodes have color c
    // node_color[i] is the color of node i
    fn bfs(&mut self, mp: &Vec<Vec<usize>>, color: &mut Vec<usize>, node_color: &mut Vec<usize>) {
        let n = mp.len();
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();

        // Start BFS from node 0 with color 0
        queue.push_back((0, 0));

        while let Some((i, c)) = queue.pop_front() {
            if !visited[i] {
                visited[i] = true;
                node_color[i] = c;
                color[c] += 1; // Increment the count of nodes with color c

                // Push adjacent nodes into the queue, flipping color (0->1 or 1->0)
                for &adj in &mp[i] {
                    if !visited[adj] {
                        queue.push_back((adj, (c + 1) % 2));
                    }
                }
            }
        }
    }

    // Main logic of the problem
    fn max_target_nodes(
        &mut self,
        edges1: &Vec<Vec<usize>>,
        edges2: &Vec<Vec<usize>>
    ) -> Vec<usize> {
        let n = edges1.len() + 1; // number of nodes in tree1
        let m = edges2.len() + 1; // number of nodes in tree2

        // Prepare vectors to hold coloring info
        self.node_color1 = vec![0; n];
        self.node_color2 = vec![0; m];
        self.color1 = vec![0; 2];
        self.color2 = vec![0; 2];

        // Build adjacency lists
        self.build(edges1, &mut self.tree1);
        self.build(edges2, &mut self.tree2);

        // BFS to color both trees
        self.bfs(&self.tree1, &mut self.color1, &mut self.node_color1);
        self.bfs(&self.tree2, &mut self.color2, &mut self.node_color2);

        // Compute the result:
        // For each node i in tree1, the result is:
        // number of nodes in tree1 that have the same color as i (color1[node_color1[i]])
        // plus the maximum of the two color counts in tree2.
        let mx = cmp::max(self.color2[0], self.color2[1]);
        let mut arr = vec![0; n];

        for i in 0..n {
            arr[i] = self.color1[self.node_color1[i]] + mx;
        }

        arr
    }
}

fn main() {
    // Read from stdin line by line
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of nodes in first tree (n1)
    let n1: usize = lines
        .next()
        .expect("Expected a line for n1")
        .unwrap()
        .trim()
        .parse()
        .expect("Expected n1 to be an integer");

    // Read edges for the first tree
    let mut edges1 = vec![vec![0; 2]; n1 - 1];
    for i in 0..(n1 - 1) {
        let line = lines
            .next()
            .expect("Expected a line for edges1")
            .unwrap();
        let mut parts = line.split_whitespace();
        let e0 = parts.next().expect("Need 2 ints per edge").parse::<usize>().expect("Bad edge format");
        let e1 = parts.next().expect("Need 2 ints per edge").parse::<usize>().expect("Bad edge format");
        edges1[i][0] = e0;
        edges1[i][1] = e1;
    }

    // Read number of nodes in second tree (n2)
    let n2: usize = lines
        .next()
        .expect("Expected a line for n2")
        .unwrap()
        .trim()
        .parse()
        .expect("Expected n2 to be an integer");

    // Read edges for the second tree
    let mut edges2 = vec![vec![0; 2]; n2 - 1];
    for i in 0..(n2 - 1) {
        let line = lines
            .next()
            .expect("Expected a line for edges2")
            .unwrap();
        let mut parts = line.split_whitespace();
        let e0 = parts.next().expect("Need 2 ints per edge").parse::<usize>().expect("Bad edge format");
        let e1 = parts.next().expect("Need 2 ints per edge").parse::<usize>().expect("Bad edge format");
        edges2[i][0] = e0;
        edges2[i][1] = e1;
    }

    // Create the solution object
    let mut solution = Solution::new();

    // Compute the result
    let result = solution.max_target_nodes(&edges1, &edges2);

    // Output results in the exact same format as the C++ code
    for val in result {
        print!("{} ", val);
    }
    println!();
}