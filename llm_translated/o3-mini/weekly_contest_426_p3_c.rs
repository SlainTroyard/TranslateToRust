use std::io::{self, Read};

// Recursive function to calculate the number of ways to walk in k steps in a tree.
// "pots" represents the adjacency list and "visited" is used to avoid going back to the parent node.
fn linepots(k: i32, pots: &Vec<Vec<i32>>, node: usize, visited: i32) -> i32 {
    if k == -1 {
        return 0; // Base case: no steps left (should not count further)
    }
    if k == 0 {
        return 1; // Base case: exactly 0 steps left, count the current path (node)
    }
    let mut answer = 1; // Count the current node
    // Iterate over all adjacent nodes
    for &neighbor in &pots[node] {
        if neighbor != visited {
            // Recursive call with k-1 steps and current node becomes the visited (parent) node.
            answer += linepots(k - 1, pots, neighbor as usize, node as i32);
        }
    }
    answer
}

// Function to calculate the maximum target nodes from tree1 combined with maximum walking paths from tree2.
fn max_target_nodes(
    edges1: &Vec<(i32, i32)>,
    edges2: &Vec<(i32, i32)>,
    k: i32,
) -> Vec<i32> {
    // Find the maximum node number in tree1 (only checking the second value as in original C code)
    let mut len1 = 0;
    for &(_, v) in edges1.iter() {
        if v > len1 {
            len1 = v;
        }
    }
    // Find the maximum node number in tree2 (only checking the second value as in original C code)
    let mut len2 = 0;
    for &(_, v) in edges2.iter() {
        if v > len2 {
            len2 = v;
        }
    }

    // Build the adjacency list for tree 1.
    // For each node from 0 to len1, add all adjacent nodes (neighbors),
    // considering both directions since the tree is undirected.
    let pots1: Vec<Vec<i32>> = (0..=len1)
        .map(|i| {
            let mut adj = Vec::new();
            for &(u, v) in edges1 {
                if u == i {
                    adj.push(v);
                }
                if v == i {
                    adj.push(u);
                }
            }
            adj
        })
        .collect();

    // Build the adjacency list for tree 2.
    let pots2: Vec<Vec<i32>> = (0..=len2)
        .map(|i| {
            let mut adj = Vec::new();
            for &(u, v) in edges2 {
                if u == i {
                    adj.push(v);
                }
                if v == i {
                    adj.push(u);
                }
            }
            adj
        })
        .collect();

    // Find the maximum number of ways to walk in (k-1) steps from any node in tree 2.
    let mut max_val = 0;
    for i in 0..=len2 {
        let temp = linepots(k - 1, &pots2, i as usize, -1);
        if temp > max_val {
            max_val = temp;
        }
    }

    // For each node in tree1, calculate the total number of paths by
    // adding the paths obtained in tree1 (using k steps) and the max from tree2.
    (0..=len1)
        .map(|i| linepots(k, &pots1, i as usize, -1) + max_val)
        .collect()
}

fn main() -> io::Result<()> {
    // Read the entire input into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    // Split the input by whitespace to get an iterator over tokens.
    let mut tokens = input.split_whitespace();

    // Parse tree1 edges.
    // The first integer is n1: the number of edges for the first tree.
    let n1: usize = tokens
        .next()
        .expect("Expected number of edges for tree1")
        .parse()
        .expect("Invalid integer for tree1 edge count");

    // Read n1 pairs of integers representing the edges.
    let mut edges1 = Vec::with_capacity(n1);
    for _ in 0..n1 {
        let u: i32 = tokens
            .next()
            .expect("Expected edge value for tree1")
            .parse()
            .expect("Invalid integer in tree1 edge");
        let v: i32 = tokens
            .next()
            .expect("Expected edge value for tree1")
            .parse()
            .expect("Invalid integer in tree1 edge");
        edges1.push((u, v));
    }

    // Parse tree2 edges.
    // The next integer is n2: the number of edges for the second tree.
    let n2: usize = tokens
        .next()
        .expect("Expected number of edges for tree2")
        .parse()
        .expect("Invalid integer for tree2 edge count");

    // Read n2 pairs of integers representing the edges.
    let mut edges2 = Vec::with_capacity(n2);
    for _ in 0..n2 {
        let u: i32 = tokens
            .next()
            .expect("Expected edge value for tree2")
            .parse()
            .expect("Invalid integer in tree2 edge");
        let v: i32 = tokens
            .next()
            .expect("Expected edge value for tree2")
            .parse()
            .expect("Invalid integer in tree2 edge");
        edges2.push((u, v));
    }

    // Read k, the number of steps.
    let k: i32 = tokens
        .next()
        .expect("Expected integer k")
        .parse()
        .expect("Invalid integer for k");

    // Calculate the result by calling max_target_nodes
    let result = max_target_nodes(&edges1, &edges2, k);

    // Output the result in the same format as original C code:
    // each integer separated by a space, followed by a newline.
    let mut output = String::new();
    for num in result {
        output.push_str(&format!("{} ", num));
    }
    // Print the result to stdout.
    println!("{}", output.trim_end());

    Ok(())
}