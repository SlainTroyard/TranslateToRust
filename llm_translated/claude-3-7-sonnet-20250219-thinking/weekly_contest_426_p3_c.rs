use std::io;
use std::cmp;

// Function to calculate the number of ways to walk in k steps
fn linepots(k: i32, pots: &Vec<Vec<usize>>, node: usize, visited: i32) -> i32 {
    if k == -1 {
        return 0; // Base case: no steps left
    }
    if k == 0 {
        return 1; // Base case: exactly 0 steps left, count 1 path
    }
    
    let mut answer = 1; // Count the current node itself
    for &neighbor in &pots[node] {
        if neighbor as i32 != visited {
            // Recursively count paths from adjacent nodes
            answer += linepots(k - 1, pots, neighbor, node as i32);
        }
    }
    return answer; // Return total paths from this node
}

// Function to calculate the maximum target nodes in two trees
fn max_target_nodes(
    edges1: &Vec<Vec<i32>>, 
    edges2: &Vec<Vec<i32>>, 
    k: i32
) -> Vec<i32> {
    // Find the maximum node number in each tree
    let len1 = edges1.iter().flatten().fold(0, |acc, &x| cmp::max(acc, x)) as usize;
    let len2 = edges2.iter().flatten().fold(0, |acc, &x| cmp::max(acc, x)) as usize;
    
    // Initialize adjacency lists
    let mut pots1: Vec<Vec<usize>> = vec![Vec::new(); len1 + 1];
    let mut pots2: Vec<Vec<usize>> = vec![Vec::new(); len2 + 1];
    let mut answer: Vec<i32> = vec![0; len1 + 1];
    
    // Build adjacency list for tree 1
    for edge in edges1 {
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        pots1[from].push(to);
        pots1[to].push(from);
    }
    
    // Build adjacency list for tree 2
    for edge in edges2 {
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        pots2[from].push(to);
        pots2[to].push(from);
    }
    
    // Find the max number of ways to walk in k-1 steps from any node in tree 2
    let mut max = 0;
    for i in 0..=len2 {
        let temp = linepots(k - 1, &pots2, i, -1);
        if temp > max {
            max = temp;
        }
    }
    
    // For each node in tree 1, calculate the total number of paths by adding paths from tree 2
    for i in 0..=len1 {
        answer[i] = linepots(k, &pots1, i, -1) + max;
    }
    
    answer
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    
    // Input for edges1
    stdin.read_line(&mut buffer)?;
    let n1: usize = buffer.trim().parse().expect("Invalid input for n1");
    buffer.clear();
    
    let mut edges1: Vec<Vec<i32>> = Vec::with_capacity(n1);
    for _ in 0..n1 {
        stdin.read_line(&mut buffer)?;
        let edge: Vec<i32> = buffer
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid edge input"))
            .collect();
        edges1.push(edge);
        buffer.clear();
    }
    
    // Input for edges2
    stdin.read_line(&mut buffer)?;
    let n2: usize = buffer.trim().parse().expect("Invalid input for n2");
    buffer.clear();
    
    let mut edges2: Vec<Vec<i32>> = Vec::with_capacity(n2);
    for _ in 0..n2 {
        stdin.read_line(&mut buffer)?;
        let edge: Vec<i32> = buffer
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid edge input"))
            .collect();
        edges2.push(edge);
        buffer.clear();
    }
    
    // Input for k
    stdin.read_line(&mut buffer)?;
    let k: i32 = buffer.trim().parse().expect("Invalid input for k");
    
    // Call the max_target_nodes function
    let result = max_target_nodes(&edges1, &edges2, k);
    
    // Output the result
    let output: String = result
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", output);
    
    Ok(())
}