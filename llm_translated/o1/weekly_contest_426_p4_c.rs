use std::io::{self, BufRead};
use std::collections::VecDeque;

/// Build an adjacency list for a tree given its edges.
fn build_adjacency_list(edges: &[(usize, usize)], n: usize) -> Vec<Vec<usize>> {
    let mut adj_list = vec![Vec::new(); n];

    // Fill adjacency list
    for &(u, v) in edges {
        adj_list[u].push(v);
        adj_list[v].push(u);
    }

    adj_list
}

/// Perform a BFS on the tree starting from node 0 to count how many nodes are colored with 0 or 1.
/// Also returns the color assigned to each node (node_color).
fn bfs_count(adj_list: &[Vec<usize>]) -> ([usize; 2], Vec<usize>) {
    let n = adj_list.len();

    // Track visited nodes
    let mut visited = vec![false; n];
    // Queue for BFS
    let mut queue = VecDeque::new();
    // Store the color for each node (0 or 1)
    let mut node_color = vec![0; n];
    // Count of how many nodes have color 0 and color 1
    let mut color_count = [0_usize; 2];

    // Start BFS from node 0
    visited[0] = true;
    queue.push_back(0);
    node_color[0] = 0;

    while let Some(curr) = queue.pop_front() {
        let color = node_color[curr];
        color_count[color] += 1;

        for &neighbor in &adj_list[curr] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                node_color[neighbor] = 1 - color; // alternate color
                queue.push_back(neighbor);
            }
        }
    }

    (color_count, node_color)
}

/// Main solution function translating the logic of C's maxTargetNodes.
fn max_target_nodes(
    edges1: &[(usize, usize)], 
    edges2: &[(usize, usize)]
) -> Vec<usize> {
    // Number of nodes in each tree
    let n1 = edges1.len() + 1;
    let n2 = edges2.len() + 1;

    // Build adjacency lists
    let adj_list1 = build_adjacency_list(edges1, n1);
    let adj_list2 = build_adjacency_list(edges2, n2);

    // Perform BFS on both trees
    let (color_count1, node_color1) = bfs_count(&adj_list1);
    let (color_count2, _node_color2) = bfs_count(&adj_list2);

    // Pick the maximum color count in tree2
    let max_color_in_tree2 = if color_count2[0] > color_count2[1] {
        color_count2[0]
    } else {
        color_count2[1]
    };

    // Calculate the result for each node in tree1
    let mut result = vec![0; n1];
    for i in 0..n1 {
        let color = node_color1[i];
        result[i] = color_count1[color] + max_color_in_tree2;
    }

    result
}

/// Read a single line from stdin and parse it into one integer (usize).
fn read_one_usize_line(lines: &mut impl Iterator<Item = io::Result<String>>) -> io::Result<usize> {
    let line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing line"))??;
    let val = line.trim().parse::<usize>()?;
    Ok(val)
}

/// Read a single line from stdin and parse it into two integers (usize).
fn read_edge_line(lines: &mut impl Iterator<Item = io::Result<String>>) -> io::Result<(usize, usize)> {
    let line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing edge line"))??;
    let nums: Vec<usize> = line
        .split_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<_, _>>()?;
    if nums.len() != 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected two integers per edge line"));
    }
    Ok((nums[0], nums[1]))
}

/// Main function replicating the original C program's I/O format
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of edges for tree 1
    let n1 = read_one_usize_line(&mut lines)?;

    // Read the edges for tree 1
    let mut edges1 = Vec::with_capacity(n1);
    for _ in 0..n1 {
        edges1.push(read_edge_line(&mut lines)?);
    }

    // Read number of edges for tree 2
    let n2 = read_one_usize_line(&mut lines)?;

    // Read the edges for tree 2
    let mut edges2 = Vec::with_capacity(n2);
    for _ in 0..n2 {
        edges2.push(read_edge_line(&mut lines)?);
    }

    // Compute the result using the translated logic
    let result = max_target_nodes(&edges1, &edges2);

    // Output the result (space separated, trailing space included, then newline)
    for val in &result {
        print!("{} ", val);
    }
    println!();

    Ok(())
}