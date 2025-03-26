use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

// Define the Solution struct to hold tree and color information
struct Solution {
    tree1: Vec<Vec<i32>>, // Adjacency list for the first tree
    tree2: Vec<Vec<i32>>, // Adjacency list for the second tree
    color1: Vec<i32>,    // Color counts for the first tree (indexed by color 0 and 1)
    color2: Vec<i32>,    // Color counts for the second tree (indexed by color 0 and 1)
    node_color1: Vec<i32>, // Node colors for the first tree
    node_color2: Vec<i32>, // Node colors for the second tree
}

impl Solution {
    // Constructor for Solution struct
    fn new() -> Self {
        Solution {
            tree1: Vec::new(),
            tree2: Vec::new(),
            color1: Vec::new(),
            color2: Vec::new(),
            node_color1: Vec::new(),
            node_color2: Vec::new(),
        }
    }

    // Builds the adjacency list representation of a tree from edge list
    fn build(&self, edges: &Vec<Vec<i32>>, mp: &mut Vec<Vec<i32>>) {
        mp.resize((edges.len() + 1) as usize); // Resize adjacency list to accommodate all nodes
        for e in edges {
            // For each edge (u, v), add v to u's adjacency list and u to v's adjacency list
            mp[e[0] as usize].push(e[1]);
            mp[e[1] as usize].push(e[0]);
        }
    }

    // Performs Breadth-First Search to color the tree nodes with alternating colors (0 and 1)
    fn bfs(&self, mp: &Vec<Vec<i32>>, color: &mut Vec<i32>, node_color: &mut Vec<i32>) {
        let n = mp.len(); // Number of nodes in the tree
        let mut q = VecDeque::new(); // Queue for BFS
        let mut vis = vec![false; n]; // Visited array to track visited nodes
        q.push_back((0, 0)); // Start BFS from node 0 with color 0
        while let Some((u, c)) = q.pop_front() {
            vis[u as usize] = true; // Mark current node as visited
            node_color[u as usize] = c; // Assign color to the current node
            color[c as usize] += 1;    // Increment the count for the current color
            for &v in &mp[u as usize] { // Iterate over neighbors of current node
                if !vis[v as usize] {   // If neighbor is not visited
                    q.push_back((v, (c + 1) % 2)); // Enqueue neighbor with the alternate color
                }
            }
        }
    }

    // Calculates the maximum target nodes based on the two input trees
    pub fn max_target_nodes(&mut self, edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges1.len() + 1; // Number of nodes in the first tree
        let m = edges2.len() + 1; // Number of nodes in the second tree
        self.node_color1.resize(n, 0); // Initialize node colors for the first tree
        self.node_color2.resize(m, 0); // Initialize node colors for the second tree
        self.color1.resize(2, 0);    // Initialize color counts for the first tree
        self.color2.resize(2, 0);    // Initialize color counts for the second tree

        // Build adjacency lists for both trees
        self.build(&edges1, &mut self.tree1);
        self.build(&edges2, &mut self.tree2);

        // Color the nodes of both trees using BFS
        self.bfs(&self.tree1, &mut self.color1, &mut self.node_color1);
        self.bfs(&self.tree2, &mut self.color2, &mut self.node_color2);

        let mut arr = vec![0; n]; // Initialize result array
        let mx = std::cmp::max(self.color2[0], self.color2[1]); // Find the maximum color count in the second tree
        for i in 0..n {
            // Calculate the target nodes for each node in the first tree
            arr[i] = self.color1[self.node_color1[i] as usize] + mx;
        }
        arr // Return the result array
    }
}


fn main() {
    let mut solution = Solution::new(); // Create a new Solution instance
    let stdin = io::stdin(); // Get standard input
    let mut iterator = stdin.lock().lines(); // Create an iterator over input lines

    // Input for edges1
    let n1: i32 = iterator.next().unwrap().unwrap().trim().parse().unwrap(); // Read number of nodes in tree1
    let mut edges1: Vec<Vec<i32>> = Vec::new(); // Initialize edge list for tree1
    for _ in 0..(n1 - 1) {
        let line = iterator.next().unwrap().unwrap(); // Read edge line
        let mut parts = line.split_whitespace(); // Split line by whitespace
        let u: i32 = parts.next().unwrap().parse().unwrap(); // Parse first node of edge
        let v: i32 = parts.next().unwrap().parse().unwrap(); // Parse second node of edge
        edges1.push(vec![u, v]); // Add edge to edge list
    }

    // Input for edges2
    let n2: i32 = iterator.next().unwrap().unwrap().trim().parse().unwrap(); // Read number of nodes in tree2
    let mut edges2: Vec<Vec<i32>> = Vec::new(); // Initialize edge list for tree2
    for _ in 0..(n2 - 1) {
        let line = iterator.next().unwrap().unwrap(); // Read edge line
        let mut parts = line.split_whitespace(); // Split line by whitespace
        let u: i32 = parts.next().unwrap().parse().unwrap(); // Parse first node of edge
        let v: i32 = parts.next().unwrap().parse().unwrap(); // Parse second node of edge
        edges2.push(vec![u, v]); // Add edge to edge list
    }

    // Call the solution method to get the result
    let result = solution.max_target_nodes(edges1, edges2);

    // Output the result in the required format
    for val in result {
        print!("{} ", val); // Print each value followed by a space
    }
    println!(); // Print a newline at the end
}