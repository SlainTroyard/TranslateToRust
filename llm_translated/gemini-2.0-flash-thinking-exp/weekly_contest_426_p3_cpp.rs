use std::io;
use std::cmp;

// Definition of the Solution struct
struct Solution {}

// Implementation block for the Solution struct
impl Solution {
    // Function to build the graph from edges
    fn build_graph(edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut g = vec![Vec::new(); edges.len() + 1]; // Initialize adjacency list
        for e in edges {
            g[e[0] as usize].push(e[1]); // Add edge from e[0] to e[1]
            g[e[1] as usize].push(e[0]); // Add edge from e[1] to e[0] (undirected graph)
        }
        g
    }

    // Depth First Search function to count nodes within distance k
    fn dfs(g: &Vec<Vec<i32>>, root: i32, par: i32, k: i32) -> i32 {
        if k < 0 {
            return 0; // If distance k becomes negative, return 0
        }
        let mut count = 1; // Initialize count with 1 (for the root node itself)
        for &node in &g[root as usize] { // Iterate over neighbors of root
            if node != par { // Avoid going back to the parent node
                count += Self::dfs(g, node, root, k - 1); // Recursively call DFS for neighbors with reduced distance
            }
        }
        count // Return the total count of nodes within distance k
    }

    // Main function to calculate max target nodes
    fn max_target_nodes(edges1: &Vec<Vec<i32>>, edges2: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let g1 = Self::build_graph(edges1); // Build graph from edges1
        let g2 = Self::build_graph(edges2); // Build graph from edges2
        let mut count = 0; // Initialize count for max nodes in g2
        let n = edges1.len() + 1; // Number of nodes in graph g1
        let m = edges2.len() + 1; // Number of nodes in graph g2
        let mut ans = Vec::new(); // Initialize vector to store results

        // Find the maximum reachable nodes in g2 with distance k-1 from each node
        for i in 0..m {
            count = cmp::max(count, Self::dfs(&g2, i as i32, -1, k - 1));
        }

        // Calculate the final answer for each node in g1 by adding max count from g2 and dfs count from g1
        for i in 0..n {
            ans.push(count + Self::dfs(&g1, i as i32, -1, k));
        }
        ans // Return the final answer vector
    }
}

fn main() {
    let solution = Solution {};

    // Input for edges1
    let mut n1_str = String::new();
    io::stdin().read_line(&mut n1_str).unwrap(); // Read n1 from input
    let n1: i32 = n1_str.trim().parse().unwrap(); // Parse n1 as integer
    let mut edges1: Vec<Vec<i32>> = vec![vec![0; 2]; n1 as usize]; // Initialize edges1 vector
    for i in 0..n1 as usize {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap(); // Read line for edge
        let nums: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect(); // Parse edge values
        edges1[i][0] = nums[0];
        edges1[i][1] = nums[1]; // Store edge values in edges1
    }

    // Input for edges2
    let mut n2_str = String::new();
    io::stdin().read_line(&mut n2_str).unwrap(); // Read n2 from input
    let n2: i32 = n2_str.trim().parse().unwrap(); // Parse n2 as integer
    let mut edges2: Vec<Vec<i32>> = vec![vec![0; 2]; n2 as usize]; // Initialize edges2 vector
    for i in 0..n2 as usize {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap(); // Read line for edge
        let nums: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect(); // Parse edge values
        edges2[i][0] = nums[0];
        edges2[i][1] = nums[1]; // Store edge values in edges2
    }

    // Input for k
    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).unwrap(); // Read k from input
    let k: i32 = k_str.trim().parse().unwrap(); // Parse k as integer

    // Call the solution method
    let result = solution.max_target_nodes(&edges1, &edges2, k);

    // Output the result
    for val in result {
        print!("{} ", val); // Print each value in result vector
    }
    println!(); // Print newline at the end
}