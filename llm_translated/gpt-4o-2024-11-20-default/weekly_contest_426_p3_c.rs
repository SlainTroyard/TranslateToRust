use std::io;
use std::collections::HashMap;

// Function to calculate the number of ways to walk in k steps
fn linepots(k: i32, pots: &HashMap<usize, Vec<usize>>, node: usize, visited: Option<usize>) -> i32 {
    if k == -1 {
        return 0; // Base case: no steps left
    }
    if k == 0 {
        return 1; // Base case: exactly 0 steps left, count 1 path
    }
    let mut answer = 1; // Count the current node itself
    if let Some(neighbors) = pots.get(&node) {
        for &next_node in neighbors {
            if Some(next_node) != visited {
                // Recursively count paths from adjacent nodes
                answer += linepots(k - 1, pots, next_node, Some(node));
            }
        }
    }
    answer // Return total paths from this node
}

// Function to calculate the maximum target nodes in two trees
fn max_target_nodes(
    edges1: &[Vec<usize>],
    edges2: &[Vec<usize>],
    k: i32,
) -> Vec<i32> {
    // Find the maximum node number in each tree
    let len1 = edges1.iter().flatten().max().copied().unwrap_or(0);
    let len2 = edges2.iter().flatten().max().copied().unwrap_or(0);

    // Build adjacency list for tree 1
    let mut pots1 = HashMap::new();
    for i in 0..=len1 {
        pots1.insert(i, Vec::new());
    }
    for edge in edges1 {
        pots1.get_mut(&edge[0]).unwrap().push(edge[1]);
        pots1.get_mut(&edge[1]).unwrap().push(edge[0]);
    }

    // Build adjacency list for tree 2
    let mut pots2 = HashMap::new();
    for i in 0..=len2 {
        pots2.insert(i, Vec::new());
    }
    for edge in edges2 {
        pots2.get_mut(&edge[0]).unwrap().push(edge[1]);
        pots2.get_mut(&edge[1]).unwrap().push(edge[0]);
    }

    // Find the max number of ways to walk in k-1 steps from any node in tree 2
    let mut max = 0;
    for i in 0..=len2 {
        let temp = linepots(k - 1, &pots2, i, None);
        if temp > max {
            max = temp;
        }
    }

    // For each node in tree 1, calculate the total number of paths by adding paths from tree 2
    let mut answer = vec![0; len1 + 1];
    for i in 0..=len1 {
        answer[i] = linepots(k, &pots1, i, None) + max;
    }

    answer
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();

    // Input edges1
    stdin.read_line(&mut input).expect("Failed to read n1");
    let n1: usize = input.trim().parse().expect("Failed to parse n1");
    let mut edges1 = Vec::new();
    for _ in 0..n1 {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read edge1");
        let edge: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse edge1"))
            .collect();
        edges1.push(edge);
    }

    // Input edges2
    input.clear();
    stdin.read_line(&mut input).expect("Failed to read n2");
    let n2: usize = input.trim().parse().expect("Failed to parse n2");
    let mut edges2 = Vec::new();
    for _ in 0..n2 {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read edge2");
        let edge: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse edge2"))
            .collect();
        edges2.push(edge);
    }

    // Input k
    input.clear();
    stdin.read_line(&mut input).expect("Failed to read k");
    let k: i32 = input.trim().parse().expect("Failed to parse k");

    // Compute the result
    let result = max_target_nodes(&edges1, &edges2, k);

    // Output the result
    for x in result {
        print!("{} ", x);
    }
    println!();
}