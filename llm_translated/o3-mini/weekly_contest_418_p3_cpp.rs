use std::io::{self, BufRead, Write};
use std::error::Error;

// Define the Solution struct (empty, but used for namespacing)
struct Solution;

impl Solution {
    // Constructs the grid layout from n and the list of edges.
    // Following the algorithm logic from the C++ code.
    fn construct_grid_layout(n: usize, edges: &[Vec<usize>]) -> Vec<Vec<usize>> {
        // Build an undirected graph represented as an adjacency list.
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
        for edge in edges {
            // Each edge has two nodes: x and y.
            // Add each node into the other's adjacency list.
            let x = edge[0];
            let y = edge[1];
            g[x].push(y);
            g[y].push(x);
        }

        // Find the node with the smallest degree - choose it as the starting point.
        let mut x = 0;
        for i in 0..n {
            if g[i].len() < g[x].len() {
                x = i;
            }
        }

        // Initialize the first row of the grid with the starting node.
        let mut row = vec![x];
        let mut vis = vec![false; n];
        vis[x] = true;
        // `deg_st` is the degree of the starting node,
        // used to decide when to stop expanding the first row.
        let deg_st = g[x].len();

        // Use a loop to traverse along neighbors to build the first row.
        // The loop continues so long as the current node's degree is greater than deg_st.
        loop {
            let mut nxt: Option<usize> = None;
            // Look at each neighbor of x.
            for &y in &g[x] {
                // If the neighbor hasn't been visited and either nxt is not chosen
                // or its degree is smaller than that of the current candidate, update nxt.
                if !vis[y] && (nxt.is_none() || g[y].len() < g[nxt.unwrap()].len()) {
                    nxt = Some(y);
                }
            }
            // If there's no valid next node, break.
            let nxt = match nxt {
                Some(val) => val,
                None => break,
            };
            // Update x, mark as visited, and push into the current row.
            x = nxt;
            row.push(x);
            vis[x] = true;
            // Continue the loop if degree is still greater than deg_st, else break.
            if g[x].len() <= deg_st {
                break;
            }
        }

        // Determine number of rows in the grid.
        // In the original C++ code, the number of rows is computed as n / row.len().
        let num_rows = n / row.len();
        let mut ans: Vec<Vec<usize>> = vec![Vec::new(); num_rows];
        // The first row is already found.
        ans[0] = row;

        // For subsequent rows, for each node in the previous row,
        // find one unvisited neighbor and assign it to the current row.
        for i in 1..num_rows {
            for &x in &ans[i - 1] {
                for &y in &g[x] {
                    if !vis[y] {
                        vis[y] = true;
                        ans[i].push(y);
                        break; // Only choose one neighbor per x.
                    }
                }
            }
        }
        ans
    }
}

// A helper function to parse input tokens
fn read_tokens() -> Result<Vec<String>, Box<dyn Error>> {
    // Read all input from stdin.
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    for line in stdin.lock().lines() {
        // Ensure we trim and split by whitespace.
        for token in line?.trim().split_whitespace() {
            tokens.push(token.to_string());
        }
    }
    Ok(tokens)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create a writer to stdout for efficient output.
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    // Read all tokens from standard input.
    let tokens = read_tokens()?;
    let mut index = 0;

    // Parse n and edgesSize exactly as in the original C++ code.
    let n: usize = tokens.get(index)
        .ok_or("Missing n")?
        .parse()?;
    index += 1;
    let edges_size: usize = tokens.get(index)
        .ok_or("Missing edges size")?
        .parse()?;
    index += 1;

    // Read in the edges.
    let mut edges = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        // Each edge is a vector of 2 numbers.
        let a: usize = tokens.get(index)
            .ok_or("Missing edge element")?
            .parse()?;
        index += 1;
        let b: usize = tokens.get(index)
            .ok_or("Missing edge element")?
            .parse()?;
        index += 1;
        edges.push(vec![a, b]);
    }

    // Create a Solution and construct the grid layout.
    let ans = Solution::construct_grid_layout(n, &edges);

    // Print out the result in the same format as the original C++ code.
    for row in ans {
        for x in row {
            write!(out, "{} ", x)?;
        }
        writeln!(out)?;
    }
    out.flush()?;
    Ok(())
}