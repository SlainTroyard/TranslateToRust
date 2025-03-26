use std::error::Error;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    // Read entire input into a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Split input by whitespace to support multiple lines or multiple values per line.
    let mut tokens = input.split_whitespace();

    // Parse the first three values: n, k, and the size of invocations.
    let n: usize = tokens
        .next()
        .ok_or("Missing number of methods (n)")?
        .parse()?;
    let k: usize = tokens
        .next()
        .ok_or("Missing starting method (k)")?
        .parse()?;
    let invocations_size: usize = tokens
        .next()
        .ok_or("Missing size of invocations")?
        .parse()?;

    // Build the graph as an adjacency list.
    // Each method is represented by its index (0 to n-1).
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    // Also store the invocations (edges) for later checking.
    let mut invocations: Vec<(usize, usize)> = Vec::with_capacity(invocations_size);
    
    for _ in 0..invocations_size {
        let from: usize = tokens
            .next()
            .ok_or("Missing invocation source value")?
            .parse()?;
        let to: usize = tokens
            .next()
            .ok_or("Missing invocation destination value")?
            .parse()?;
        graph[from].push(to);
        invocations.push((from, to));
    }

    // Mark all suspicious methods by performing DFS starting from method k.
    let mut is_suspicious = vec![false; n];
    
    // Define a recursive DFS function.
    fn dfs(x: usize, graph: &Vec<Vec<usize>>, is_suspicious: &mut Vec<bool>) {
        is_suspicious[x] = true;
        for &y in &graph[x] {
            if !is_suspicious[y] {
                dfs(y, graph, is_suspicious);
            }
        }
    }
    dfs(k, &graph, &mut is_suspicious);

    // Check if there is any edge from a non-suspicious method to a suspicious one.
    // If such an edge exists, we cannot remove any suspicious methods.
    let mut edge_from_non_suspicious_to_suspicious = false;
    for &(from, to) in &invocations {
        if !is_suspicious[from] && is_suspicious[to] {
            edge_from_non_suspicious_to_suspicious = true;
            break;
        }
    }
    
    // Determine the answer.
    let ans: Vec<usize> = if edge_from_non_suspicious_to_suspicious {
        // Cannot remove suspicious methods: return all method indices.
        (0..n).collect()
    } else {
        // Otherwise, remove all suspicious methods and keep the rest.
        (0..n).filter(|&i| !is_suspicious[i]).collect()
    };

    // Print the answer with the exact same output format.
    // Each method index is printed separated by a space, followed by a newline.
    let stdout = io::stdout();
    let mut out = stdout.lock();
    for method in ans {
        write!(out, "{} ", method)?;
    }
    writeln!(out)?;
    
    Ok(())
}