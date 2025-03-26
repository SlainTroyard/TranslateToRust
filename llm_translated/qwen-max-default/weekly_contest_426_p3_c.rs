use std::io::{self, BufRead};

// Function to calculate the number of ways to walk in k steps
fn linepots(k: i32, pots: &Vec<Vec<usize>>, node: usize, length: &Vec<usize>, visited: isize) -> i32 {
    if k == -1 {
        return 0; // Base case: no steps left
    }
    if k == 0 {
        return 1; // Base case: exactly 0 steps left, count 1 path
    }
    let mut answer = 1; // Count the current node itself
    for &neighbor in &pots[node] {
        if neighbor as isize != visited {
            answer += linepots(k - 1, pots, neighbor, length, node as isize); // Recursively count paths from adjacent nodes
        }
    }
    answer // Return total paths from this node
}

// Function to calculate the maximum target nodes in two trees
fn max_target_nodes(edges1: &Vec<(usize, usize)>, edges2: &Vec<(usize, usize)>, k: i32) -> Vec<i32> {
    let (len1, len2) = (edges1.iter().map(|e| e.1).max().unwrap_or(0), edges2.iter().map(|e| e.1).max().unwrap_or(0));

    // Allocate memory for adjacency lists and other arrays
    let mut pots1 = vec![vec![]; len1 + 1];
    let mut pots2 = vec![vec![]; len2 + 1];
    let mut length1 = vec![0; len1 + 1];
    let mut length2 = vec![0; len2 + 1];

    // Build adjacency list for tree 1
    for &(a, b) in edges1 {
        pots1[a].push(b);
        pots1[b].push(a);
        length1[a] += 1;
        length1[b] += 1;
    }

    // Build adjacency list for tree 2
    for &(a, b) in edges2 {
        pots2[a].push(b);
        pots2[b].push(a);
        length2[a] += 1;
        length2[b] += 1;
    }

    // Find the max number of ways to walk in k-1 steps from any node in tree 2
    let max = (0..=len2).map(|i| linepots(k - 1, &pots2, i, &length2, -1)).max().unwrap_or(0);

    // For each node in tree 1, calculate the total number of paths by adding paths from tree 2
    (0..=len1).map(|i| linepots(k, &pots1, i, &length1, -1) + max).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input for edges1
    let n1: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let edges1: Vec<(usize, usize)> = (0..n1)
        .map(|_| {
            let line = lines.next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    // Input for edges2
    let n2: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let edges2: Vec<(usize, usize)> = (0..n2)
        .map(|_| {
            let line = lines.next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    // Input for k
    let k: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    // Call the max_target_nodes function
    let result = max_target_nodes(&edges1, &edges2, k);

    // Output the result
    for value in result {
        print!("{} ", value);
    }
    println!();
}