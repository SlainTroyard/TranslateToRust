use std::io::{self, BufRead, Write};
use std::error::Error;

/// Perform a depth-first search (DFS) starting from node `x`
/// and mark all reached nodes in the `visited` vector.
/// 
/// # Arguments
/// 
/// * `x` - The current node index.
/// * `adj` - The adjacency list representing the graph.
/// * `visited` - Mutable boolean vector indicating if a node has been visited.
fn dfs(x: usize, adj: &[Vec<usize>], visited: &mut [bool]) {
    visited[x] = true;
    for &next in &adj[x] {
        if !visited[next] {
            dfs(next, adj, visited);
        }
    }
}

/// Process the remaining methods according to the problem logic.
/// 
/// The function builds a graph from the list of invocations, performs a DFS
/// starting from method `k` to mark suspicious methods, and then decides:
///   - If there is any invocation where a non-suspicious method calls a suspicious method,
///     return all method indices.
///   - Otherwise, return all non-suspicious method indices.
/// 
/// # Arguments
/// 
/// * `n` - Total number of methods.
/// * `k` - The starting method for the DFS (suspicious).
/// * `invocations` - A slice of tuples (a, b) representing method invocations.
/// 
/// # Returns
/// 
/// A vector of method indices based on the described logic.
fn remaining_methods(n: usize, k: usize, invocations: &[(usize, usize)]) -> Vec<usize> {
    // Build the adjacency list for the graph.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &(a, b) in invocations {
        adj[a].push(b);
    }

    // Create a vector that marks suspicious methods (visited in DFS).
    let mut suspicious = vec![false; n];

    // Run DFS starting from method k.
    dfs(k, &adj, &mut suspicious);

    // Check if there exists any invocation where a non-suspicious method calls
    // a suspicious method.
    for &(a, b) in invocations {
        if !suspicious[a] && suspicious[b] {
            // Return all methods if the condition is met.
            return (0..n).collect();
        }
    }

    // Otherwise, return only the methods that are not suspicious.
    (0..n).filter(|&i| !suspicious[i]).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Use a buffered reader for efficient input reading.
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    let mut input = String::new();

    // Read the entire input.
    reader.read_to_string(&mut input)?;

    // Split the input into tokens (whitespace separated).
    let mut tokens = input.split_whitespace();

    // The first three tokens are n, k, and invocations_size.
    let n: usize = tokens.next().ok_or("Missing n")?.parse()?;
    let k: usize = tokens.next().ok_or("Missing k")?.parse()?;
    let invocations_size: usize = tokens.next().ok_or("Missing invocations size")?.parse()?;

    // Read each invocation, which consists of two integers.
    let mut invocations = Vec::with_capacity(invocations_size);
    for _ in 0..invocations_size {
        let a: usize = tokens.next().ok_or("Missing invocation value a")?.parse()?;
        let b: usize = tokens.next().ok_or("Missing invocation value b")?.parse()?;
        invocations.push((a, b));
    }

    // Compute the answer using the problem logic.
    let ans = remaining_methods(n, k, &invocations);

    // Write the output exactly as the original C code (numbers separated by spaces, ending with a newline).
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for num in ans {
        write!(handle, "{} ", num)?;
    }
    writeln!(handle)?;

    Ok(())
}