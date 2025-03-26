use std::collections::VecDeque;
use std::io::{self, BufRead, Write};

/// The Solution struct to encapsulate the trees and color arrays.
struct Solution {
    tree1: Vec<Vec<usize>>,   // Adjacency list for tree1
    tree2: Vec<Vec<usize>>,   // Adjacency list for tree2
    color1: Vec<i32>,         // Count of nodes with color 0 and 1 in tree1
    color2: Vec<i32>,         // Count of nodes with color 0 and 1 in tree2
    node_color1: Vec<i32>,    // Color assigned to each node in tree1
    node_color2: Vec<i32>,    // Color assigned to each node in tree2
}

impl Solution {
    /// Create a new instance of Solution with empty trees and color arrays.
    fn new() -> Self {
        Self {
            tree1: Vec::new(),
            tree2: Vec::new(),
            color1: Vec::new(),
            color2: Vec::new(),
            node_color1: Vec::new(),
            node_color2: Vec::new(),
        }
    }
    
    /// Build an undirected tree from edge list.
    ///
    /// The edge list is a vector of [u, v] pairs. The resulting adjacency list `mp` will have a length of edges.len() + 1.
    fn build(&self, edges: &Vec<Vec<usize>>, mp: &mut Vec<Vec<usize>>) {
        // Resize mp to have (number of nodes) entries.
        mp.resize(edges.len() + 1, Vec::new());
        for e in edges {
            // Add edge in both directions.
            mp[e[0]].push(e[1]);
            mp[e[1]].push(e[0]);
        }
    }

    /// Perform a BFS on the given tree and assign colors alternately.
    ///
    /// The `color` vector stores the count for each color. The `node_color` vector stores each node's assigned color.
    fn bfs(&self, mp: &Vec<Vec<usize>>, color: &mut Vec<i32>, node_color: &mut Vec<i32>) {
        let n = mp.len();
        let mut q = VecDeque::new();
        let mut vis = vec![false; n];
        // Start BFS from node 0 with color 0.
        q.push_back((0, 0));
        
        while let Some((node, col)) = q.pop_front() {
            if vis[node] {
                continue;
            }
            vis[node] = true;
            node_color[node] = col;
            color[col as usize] += 1;
            // Traverse adjacent nodes.
            for &adj in &mp[node] {
                if !vis[adj] {
                    // Alternate color using (col + 1) % 2.
                    q.push_back((adj, (col + 1) % 2));
                }
            }
        }
    }

    /// Solve the problem as described:
    /// - Build two trees from edge lists.
    /// - Color tree1 and tree2 using BFS.
    /// - For each node in tree1, compute the sum of the number of nodes in tree1 with the same color as this node and
    ///   the maximum count of either color in tree2.
    fn max_target_nodes(&mut self, edges1: Vec<Vec<usize>>, edges2: Vec<Vec<usize>>) -> Vec<i32> {
        let n = edges1.len() + 1;  // Number of nodes in tree1
        let m = edges2.len() + 1;  // Number of nodes in tree2

        // Initialize node colors with dummy values
        self.node_color1 = vec![0; n];
        self.node_color2 = vec![0; m];
        // Initialize color counts (only two colors: 0 and 1)
        self.color1 = vec![0, 0];
        self.color2 = vec![0, 0];
        
        // Build the trees from edges.
        self.build(&edges1, &mut self.tree1);
        self.build(&edges2, &mut self.tree2);

        // Color the trees using BFS.
        self.bfs(&self.tree1, &mut self.color1, &mut self.node_color1);
        self.bfs(&self.tree2, &mut self.color2, &mut self.node_color2);

        // Determine the maximum number of nodes of one color in tree2.
        let mx = self.color2[0].max(self.color2[1]);
        // Compute the result for each node in tree1.
        let mut arr = vec![0; n];
        for i in 0..n {
            // For each node in tree1, add the count of nodes in tree1 with that color plus mx from tree2.
            arr[i] = self.color1[self.node_color1[i] as usize] + mx;
        }
        arr
    }
}

/// Helper function to parse input tokens from standard input.
fn read_tokens() -> Vec<String> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    reader
        .lines()
        .filter_map(Result::ok)
        .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<String>>())
        .collect()
}

fn main() {
    // Use buffered output for faster I/O.
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    // Read all tokens from standard input.
    let tokens = read_tokens();
    let mut iter = tokens.into_iter();

    // Parse first tree input.
    // Read n1, the number of nodes in tree1.
    let n1: usize = iter
        .next()
        .expect("Expected number of nodes for tree1")
        .parse()
        .expect("Failed to parse n1 as integer");

    // Tree1 has n1 - 1 edges.
    let mut edges1 = Vec::with_capacity(n1 - 1);
    for _ in 0..(n1 - 1) {
        let u: usize = iter
            .next()
            .expect("Expected node in tree1 edge")
            .parse()
            .expect("Failed to parse node in tree1 edge");
        let v: usize = iter
            .next()
            .expect("Expected node in tree1 edge")
            .parse()
            .expect("Failed to parse node in tree1 edge");
        edges1.push(vec![u, v]);
    }

    // Parse second tree input.
    // Read n2, the number of nodes in tree2.
    let n2: usize = iter
        .next()
        .expect("Expected number of nodes for tree2")
        .parse()
        .expect("Failed to parse n2 as integer");

    // Tree2 has n2 - 1 edges.
    let mut edges2 = Vec::with_capacity(n2 - 1);
    for _ in 0..(n2 - 1) {
        let u: usize = iter
            .next()
            .expect("Expected node in tree2 edge")
            .parse()
            .expect("Failed to parse node in tree2 edge");
        let v: usize = iter
            .next()
            .expect("Expected node in tree2 edge")
            .parse()
            .expect("Failed to parse node in tree2 edge");
        edges2.push(vec![u, v]);
    }

    // Create a new solution instance and get the result.
    let mut solution = Solution::new();
    let result = solution.max_target_nodes(edges1, edges2);

    // Print the output in the same format as the original C++ code.
    // Each number is printed with a space and followed by a newline at the end.
    for val in result {
        write!(out, "{} ", val).expect("Failed to write output");
    }
    writeln!(out).expect("Failed to write newline");
}