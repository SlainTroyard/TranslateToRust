use std::io::{self, Read};

fn linepots(k: i32, pots: &[Vec<usize>], node: usize, visited: i32) -> i32 {
    if k == -1 {
        return 0;
    }
    if k == 0 {
        return 1;
    }
    let mut answer = 1; // Include current node as 0 steps
    for &neighbor in &pots[node] {
        if neighbor as i32 != visited {
            answer += linepots(k - 1, pots, neighbor, node as i32);
        }
    }
    answer
}

fn max_target_nodes(edges1: &[[usize; 2]], edges2: &[[usize; 2]], k: i32) -> Vec<i32> {
    let len1 = edges1.iter().map(|e| e[1]).max().unwrap_or(0);
    let len2 = edges2.iter().map(|e| e[1]).max().unwrap_or(0);

    // Build adjacency list for tree1
    let mut pots1 = vec![vec![]; len1 + 1];
    for i in 0..=len1 {
        let mut neighbors = Vec::new();
        for &edge in edges1 {
            if edge[0] == i {
                neighbors.push(edge[1]);
            }
            if edge[1] == i {
                neighbors.push(edge[0]);
            }
        }
        pots1[i] = neighbors;
    }

    // Build adjacency list for tree2
    let mut pots2 = vec![vec![]; len2 + 1];
    for i in 0..=len2 {
        let mut neighbors = Vec::new();
        for &edge in edges2 {
            if edge[0] == i {
                neighbors.push(edge[1]);
            }
            if edge[1] == i {
                neighbors.push(edge[0]);
            }
        }
        pots2[i] = neighbors;
    }

    // Calculate max value for tree2
    let mut max = 0;
    for i in 0..=len2 {
        let temp = linepots(k - 1, &pots2, i, -1);
        if temp > max {
            max = temp;
        }
    }

    // Calculate answer for each node in tree1
    let mut answer = vec![0; len1 + 1];
    for i in 0..=len1 {
        answer[i] = linepots(k, &pots1, i, -1) + max;
    }

    answer
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<usize>().unwrap());

    // Read edges1
    let n1 = tokens.next().unwrap();
    let mut edges1 = Vec::new();
    for _ in 0..n1 {
        let u = tokens.next().unwrap();
        let v = tokens.next().unwrap();
        edges1.push([u, v]);
    }

    // Read edges2
    let n2 = tokens.next().unwrap();
    let mut edges2 = Vec::new();
    for _ in 0..n2 {
        let u = tokens.next().unwrap();
        let v = tokens.next().unwrap();
        edges2.push([u, v]);
    }

    // Read k
    let k = tokens.next().unwrap() as i32;

    // Compute result
    let result = max_target_nodes(&edges1, &edges2, k);

    // Print output
    for num in &result {
        print!("{} ", num);
    }
    println!();
}