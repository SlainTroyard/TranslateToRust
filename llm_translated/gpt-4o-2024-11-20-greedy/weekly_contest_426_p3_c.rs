// Problem: Weekly Contest 426 Problem 3
use std::io::{self, BufRead};
use std::collections::HashMap;

fn linepots(k: i32, pots: &HashMap<usize, Vec<usize>>, node: usize, visited: Option<usize>) -> i32 {
    if k == -1 {
        return 0; // Base case: no steps left
    }
    if k == 0 {
        return 1; // Base case: exactly 0 steps left, count 1 path
    }
    let mut answer = 1; // Count the current node itself
    if let Some(neighbors) = pots.get(&node) {
        for &neighbor in neighbors {
            if Some(neighbor) != visited {
                answer += linepots(k - 1, pots, neighbor, Some(node)); // Recursively count paths from adjacent nodes
            }
        }
    }
    answer // Return total paths from this node
}

fn max_target_nodes(
    edges1: Vec<(usize, usize)>,
    edges2: Vec<(usize, usize)>,
    k: i32,
) -> Vec<i32> {
    let mut len1 = 0;
    let mut len2 = 0;

    // Find the maximum node number in each tree
    for &(a, b) in &edges1 {
        len1 = len1.max(a).max(b);
    }
    for &(a, b) in &edges2 {
        len2 = len2.max(a).max(b);
    }

    // Build adjacency list for tree 1
    let mut pots1: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..=len1 {
        pots1.entry(i).or_insert_with(Vec::new);
    }
    for &(a, b) in &edges1 {
        pots1.get_mut(&a).unwrap().push(b);
        pots1.get_mut(&b).unwrap().push(a);
    }

    // Build adjacency list for tree 2
    let mut pots2: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..=len2 {
        pots2.entry(i).or_insert_with(Vec::new);
    }
    for &(a, b) in &edges2 {
        pots2.get_mut(&a).unwrap().push(b);
        pots2.get_mut(&b).unwrap().push(a);
    }

    // Find the max number of ways to walk in k-1 steps from any node in tree 2
    let mut max = 0;
    for i in 0..=len2 {
        let temp = linepots(k - 1, &pots2, i, None);
        max = max.max(temp);
    }

    // For each node in tree 1, calculate the total number of paths by adding paths from tree 2
    let mut answer = vec![0; len1 + 1];
    for i in 0..=len1 {
        answer[i] = linepots(k, &pots1, i, None) + max;
    }

    answer
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input for edges1
    let n1: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut edges1 = Vec::with_capacity(n1);
    for _ in 0..n1 {
        let edge: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        edges1.push((edge[0], edge[1]));
    }

    // Input for edges2
    let n2: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut edges2 = Vec::with_capacity(n2);
    for _ in 0..n2 {
        let edge: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        edges2.push((edge[0], edge[1]));
    }

    // Input for k
    let k: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Call the max_target_nodes function
    let result = max_target_nodes(edges1, edges2, k);

    // Output the result
    for value in &result {
        print!("{} ", value);
    }
    println!();
}