use std::io::{self, BufRead};

// Function to calculate the number of ways to walk in k steps
fn linepots(k: i32, pots: &Vec<Vec<usize>>, node: usize, length: &Vec<usize>, visited: i32) -> i32 {
    if k == -1 {
        return 0; // Base case: no steps left
    }
    if k == 0 {
        return 1; // Base case: exactly 0 steps left, count 1 path
    }
    let mut answer = 1; // Count the current node itself
    for &i in pots[node].iter().take(length[node]) {
        if i as i32 != visited {
            answer += linepots(k - 1, pots, i, length, node as i32); // Recursively count paths from adjacent nodes
        }
    }
    answer // Return total paths from this node
}

// Function to calculate the maximum target nodes in two trees
fn max_target_nodes(edges1: &Vec<Vec<usize>>, edges2: &Vec<Vec<usize>>, k: i32) -> Vec<i32> {
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

    // Build adjacency list for tree 1
    let mut pots1 = vec![Vec::new(); len1 + 1];
    let mut length1 = vec![0; len1 + 1];
    for i in 0..=len1 {
        for edge in edges1 {
            if edge[0] == i {
                pots1[i].push(edge[1]);
                length1[i] += 1;
            }
            if edge[1] == i {
                pots1[i].push(edge[0]);
                length1[i] += 1;
            }
        }
    }

    // Build adjacency list for tree 2
    let mut pots2 = vec![Vec::new(); len2 + 1];
    let mut length2 = vec![0; len2 + 1];
    for i in 0..=len2 {
        for edge in edges2 {
            if edge[0] == i {
                pots2[i].push(edge[1]);
                length2[i] += 1;
            }
            if edge[1] == i {
                pots2[i].push(edge[0]);
                length2[i] += 1;
            }
        }
    }

    // Find the max number of ways to walk in k-1 steps from any node in tree 2
    let mut max = 0;
    for i in 0..=len2 {
        let temp = linepots(k - 1, &pots2, i, &length2, -1);
        if temp > max {
            max = temp;
        }
    }

    // For each node in tree 1, calculate the total number of paths by adding paths from tree 2
    let mut answer = vec![0; len1 + 1];
    for i in 0..=len1 {
        answer[i] = linepots(k, &pots1, i, &length1, -1) + max;
    }

    answer
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input for edges1
    let n1: usize = lines.next().unwrap()?.parse().unwrap();
    let mut edges1 = Vec::with_capacity(n1);
    for _ in 0..n1 {
        let line = lines.next().unwrap()?;
        let mut nums = line.split_whitespace().map(|s| s.parse().unwrap());
        edges1.push(vec![nums.next().unwrap(), nums.next().unwrap()]);
    }

    // Input for edges2
    let n2: usize = lines.next().unwrap()?.parse().unwrap();
    let mut edges2 = Vec::with_capacity(n2);
    for _ in 0..n2 {
        let line = lines.next().unwrap()?;
        let mut nums = line.split_whitespace().map(|s| s.parse().unwrap());
        edges2.push(vec![nums.next().unwrap(), nums.next().unwrap()]);
    }

    // Input for k
    let k: i32 = lines.next().unwrap()?.parse().unwrap();

    // Call the max_target_nodes function
    let result = max_target_nodes(&edges1, &edges2, k);

    // Output the result
    for &num in &result {
        print!("{} ", num);
    }
    println!();

    Ok(())
}