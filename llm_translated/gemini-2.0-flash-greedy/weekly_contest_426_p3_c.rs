use std::io;
use std::io::Read;
use std::vec;

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
            answer += linepots(k - 1, pots, neighbor, node as i32); // Recursively count paths from adjacent nodes
        }
    }
    answer // Return total paths from this node
}

// Function to calculate the maximum target nodes in two trees
fn max_target_nodes(
    edges1: &Vec<Vec<i32>>,
    edges2: &Vec<Vec<i32>>,
    k: i32,
) -> Vec<i32> {
    let mut len1 = 0;
    let mut len2 = 0;

    // Find the maximum node number in each tree
    for edge in edges1 {
        if edge[1] > len1 {
            len1 = edge[1];
        }
    }
    for edge in edges2 {
        if edge[1] > len2 {
            len2 = edge[1];
        }
    }

    let len1 = len1 as usize;
    let len2 = len2 as usize;

    // Build adjacency list for tree 1
    let mut pots1: Vec<Vec<usize>> = vec![Vec::new(); len1 + 1];
    for edge in edges1 {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        pots1[u].push(v);
        pots1[v].push(u);
    }

    // Build adjacency list for tree 2
    let mut pots2: Vec<Vec<usize>> = vec![Vec::new(); len2 + 1];
    for edge in edges2 {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        pots2[u].push(v);
        pots2[v].push(u);
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
    let mut answer: Vec<i32> = vec![0; len1 + 1];
    for i in 0..=len1 {
        answer[i] = linepots(k, &pots1, i, -1) + max;
    }

    answer // Return the result vector
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    // Input for edges1
    let n1: usize = lines.next().unwrap().parse().unwrap();
    let mut edges1: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n1 {
        let edge_str = lines.next().unwrap();
        let mut edge_nums = edge_str.split_whitespace();
        let u: i32 = edge_nums.next().unwrap().parse().unwrap();
        let v: i32 = edge_nums.next().unwrap().parse().unwrap();
        edges1.push(vec![u, v]);
    }

    // Input for edges2
    let n2: usize = lines.next().unwrap().parse().unwrap();
    let mut edges2: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n2 {
        let edge_str = lines.next().unwrap();
        let mut edge_nums = edge_str.split_whitespace();
        let u: i32 = edge_nums.next().unwrap().parse().unwrap();
        let v: i32 = edge_nums.next().unwrap().parse().unwrap();
        edges2.push(vec![u, v]);
    }

    // Input for k
    let k: i32 = lines.next().unwrap().parse().unwrap();

    // Call the max_target_nodes function
    let result = max_target_nodes(&edges1, &edges2, k);

    // Output the result
    for &val in &result {
        print!("{} ", val);
    }
    println!();
}