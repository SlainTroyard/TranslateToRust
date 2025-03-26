use std::cmp;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Build an undirected graph represented as an adjacency list.
    // The graph's nodes are assumed to be labeled from 0 to edges.len()
    fn build_graph(&self, edges: &[Vec<i32>]) -> Vec<Vec<usize>> {
        // The number of nodes equals edges.len() + 1 (as in the original C++ code)
        let num_nodes = edges.len() + 1;
        let mut graph = vec![Vec::new(); num_nodes];
        for edge in edges {
            // Convert node labels to usize for indexing.
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            // Since the graph is undirected, add both edges.
            graph[u].push(v);
            graph[v].push(u);
        }
        graph
    }

    // Depth-first search that counts the number of nodes reachable from `root` with depth limit k.
    // `par` is an Option holding the parent node (if any).
    fn dfs(&self, graph: &Vec<Vec<usize>>, root: usize, par: Option<usize>, k: i32) -> i32 {
        // If the remaining depth is negative, no nodes can be visited.
        if k < 0 {
            return 0;
        }
        // Start count with the current node.
        let mut count = 1;
        // Traverse all adjacent nodes.
        for &neighbor in &graph[root] {
            if Some(neighbor) != par {
                count += self.dfs(graph, neighbor, Some(root), k - 1);
            }
        }
        count
    }

    // Main function that computes the maximum target nodes from two graphs.
    // It returns a vector where each element is the sum of the maximum value found from the second graph
    // and the dfs count from the first graph starting at the corresponding node.
    fn max_target_nodes(&self, edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let g1 = self.build_graph(&edges1);
        let g2 = self.build_graph(&edges2);
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        let mut count = 0;
        // Compute the maximum DFS count over all nodes in g2 with a depth limit (k-1).
        for i in 0..m {
            count = cmp::max(count, self.dfs(&g2, i, None, k - 1));
        }
        // For each node in g1, add the previously computed maximum count
        // to the DFS result starting at this node with a depth limit k.
        let mut ans = Vec::with_capacity(n);
        for i in 0..n {
            ans.push(count + self.dfs(&g1, i, None, k));
        }
        ans
    }
}

// Utility function to read tokens from standard input.
fn read_tokens() -> Vec<String> {
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    // Lock the standard input for buffered reading.
    let reader = stdin.lock();
    for line in reader.lines() {
        let line = line.expect("Failed to read input line");
        tokens.extend(line.split_whitespace().map(|s| s.to_string()));
    }
    tokens
}

fn main() {
    // Read all tokens from the standard input.
    let tokens = read_tokens();
    // Use an iterator to parse tokens sequentially.
    let mut iter = tokens.into_iter();

    // Read number of edges for edges1.
    let n1: usize = iter
        .next()
        .expect("Expected number of edges for edges1")
        .parse()
        .expect("Failed to parse n1");
    // Read edges1: expecting n1 pairs of integers.
    let mut edges1 = Vec::with_capacity(n1);
    for _ in 0..n1 {
        let u: i32 = iter
            .next()
            .expect("Expected edge starting node for edges1")
            .parse()
            .expect("Failed to parse edge value");
        let v: i32 = iter
            .next()
            .expect("Expected edge ending node for edges1")
            .parse()
            .expect("Failed to parse edge value");
        edges1.push(vec![u, v]);
    }

    // Read number of edges for edges2.
    let n2: usize = iter
        .next()
        .expect("Expected number of edges for edges2")
        .parse()
        .expect("Failed to parse n2");
    // Read edges2: expecting n2 pairs of integers.
    let mut edges2 = Vec::with_capacity(n2);
    for _ in 0..n2 {
        let u: i32 = iter
            .next()
            .expect("Expected edge starting node for edges2")
            .parse()
            .expect("Failed to parse edge value");
        let v: i32 = iter
            .next()
            .expect("Expected edge ending node for edges2")
            .parse()
            .expect("Failed to parse edge value");
        edges2.push(vec![u, v]);
    }

    // Read the integer k.
    let k: i32 = iter
        .next()
        .expect("Expected value for k")
        .parse()
        .expect("Failed to parse k");

    // Create an instance of the solution and call the max_target_nodes method.
    let solution = Solution;
    let result = solution.max_target_nodes(edges1, edges2, k);

    // Print the resulting vector.
    // The output format is: each number followed by a space, and a newline at the end.
    for val in result {
        print!("{} ", val);
    }
    println!();
}