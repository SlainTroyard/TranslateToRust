use std::io;

/// Computes the number of valid paths in k steps starting from a node without backtracking.
fn linepots(k: i32, pots: &[Vec<usize>], node: usize, prev: Option<usize>) -> i32 {
    if k == -1 {
        return 0;
    }
    if k == 0 {
        return 1;
    }
    let mut ans = 1;
    for &neighbor in &pots[node] {
        if prev.is_none() || (prev.unwrap() != neighbor) {
            ans += linepots(k - 1, pots, neighbor, Some(node));
        }
    }
    ans
}

/// Computes the maximum target nodes for two trees based on path counts.
pub fn max_target_nodes(edges1: &[[usize; 2]], edges2: &[[usize; 2]], k: i32) -> Vec<i32> {
    // Determine the maximum node in each tree's edges (only the second element is considered)
    let max_node1 = edges1.iter().map(|e| e[1]).max().unwrap_or(0);
    let max_node2 = edges2.iter().map(|e| e[1]).max().unwrap_or(0);

    // Build adjacency lists for both trees
    let mut pots1 = vec![Vec::new(); max_node1 + 1];
    for edge in edges1.iter() {
        let (u, v) = (edge[0], edge[1]);
        pots1[u].push(v);
        pots1[v].push(u);
    }

    let mut pots2 = vec![Vec::new(); max_node2 + 1];
    for edge in edges2.iter() {
        let (u, v) = (edge[0], edge[1]);
        pots2[u].push(v);
        pots2[v].push(u);
    }

    // Find the maximum path count from any node in tree2 for k-1 steps
    let mut max_val = 0;
    for node in 0..=max_node2 {
        let temp = linepots(k - 1, &pots2, node, None);
        if temp > max_val {
            max_val = temp;
        }
    }

    // Calculate result for each node in tree1 by adding max_val
    let mut answer = Vec::with_capacity(max_node1 + 1);
    for node in 0..=max_node1 {
        let current = linepots(k, &pots1, node, None) + max_val;
        answer.push(current);
    }
    answer
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // Read edges for the first tree
    let n1: usize = lines.next().unwrap().parse().unwrap();
    let mut edges1 = Vec::new();
    for _ in 0..n1 {
        let line = lines.next().unwrap();
        let nums: Vec<usize> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        edges1.push([nums[0], nums[1]]);
    }

    // Read edges for the second tree
    let n2: usize = lines.next().unwrap().parse().unwrap();
    let mut edges2 = Vec::new();
    for _ in 0..n2 {
        let line = lines.next().unwrap();
        let nums: Vec<usize> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        edges2.push([nums[0], nums[1]]);
    }

    // Read k value
    let k: i32 = lines.next().unwrap().parse().unwrap();

    // Compute and print the result
    let result = max_target_nodes(&edges1, &edges2, k);
    for num in &result {
        print!("{} ", num);
    }
    println!();
}