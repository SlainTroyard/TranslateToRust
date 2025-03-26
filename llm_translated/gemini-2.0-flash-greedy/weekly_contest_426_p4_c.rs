use std::io;
use std::io::Read;
use std::vec;

/* Function to build adjacency list for a tree */
fn build_adjacency_list(
    edges: &Vec<Vec<i32>>,
    n: usize,
) -> (Vec<Vec<i32>>, Vec<usize>) {
    let mut adj_list: Vec<Vec<i32>> = vec![Vec::new(); n];
    let mut adj_len: Vec<usize> = vec![0; n];

    // Count the degree of each node
    for edge in edges {
        adj_len[edge[0] as usize] += 1;
        adj_len[edge[1] as usize] += 1;
    }

    // Allocate memory for adjacency list and fill it
    for i in 0..n {
        adj_list[i] = Vec::with_capacity(adj_len[i]);
    }

    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj_list[u].push(v as i32);
        adj_list[v].push(u as i32);
    }

    (adj_list, adj_len)
}

/* BFS to count nodes of each color */
fn bfs_count(
    adj_list: &Vec<Vec<i32>>,
    adj_len: &Vec<usize>,
    n: usize,
    color_count: &mut [i32; 2],
    node_color: &mut Vec<i32>,
) {
    let mut visited: Vec<bool> = vec![false; n];
    let mut queue: Vec<usize> = Vec::new();
    let mut front = 0;
    let mut rear = 0;

    queue.push(0); // Start BFS from node 0
    node_color[0] = 0; // Color of root is 0
    visited[0] = true;
    rear += 1;

    while front < rear {
        let curr = queue[front];
        front += 1;
        let color = node_color[curr];
        color_count[color as usize] += 1;

        for i in 0..adj_len[curr] {
            let neighbor = adj_list[curr][i] as usize;
            if !visited[neighbor] {
                visited[neighbor] = true;
                node_color[neighbor] = 1 - color; // Alternate color
                queue.push(neighbor);
                rear += 1;
            }
        }
    }
}

/* Main solution function */
fn max_target_nodes(
    edges1: &Vec<Vec<i32>>,
    edges2: &Vec<Vec<i32>>,
) -> Vec<i32> {
    let n1 = edges1.len() + 1; // Number of nodes in tree 1
    let n2 = edges2.len() + 1; // Number of nodes in tree 2

    // Build adjacency lists
    let (adj_list1, adj_len1) = build_adjacency_list(edges1, n1);
    let (adj_list2, adj_len2) = build_adjacency_list(edges2, n2);

    // Color count and node color arrays
    let mut color_count1: [i32; 2] = [0, 0];
    let mut color_count2: [i32; 2] = [0, 0];
    let mut node_color1: Vec<i32> = vec![0; n1];
    let mut node_color2: Vec<i32> = vec![0; n2];

    // Perform BFS to calculate color distribution
    bfs_count(&adj_list1, &adj_len1, n1, &mut color_count1, &mut node_color1);
    bfs_count(&adj_list2, &adj_len2, n2, &mut color_count2, &mut node_color2);

    // Calculate results for tree 1
    let mut result: Vec<i32> = vec![0; n1];
    let max_color_in_tree2 = color_count2[0].max(color_count2[1]);

    for i in 0..n1 {
        result[i] = color_count1[node_color1[i] as usize] + max_color_in_tree2;
    }

    result
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    // Input number of edges for tree 1
    let n1: usize = lines.next().unwrap().parse().unwrap();
    let mut edges1: Vec<Vec<i32>> = Vec::new();

    for _ in 0..n1 {
        let line = lines.next().unwrap();
        let mut nums = line.split_whitespace();
        let u: i32 = nums.next().unwrap().parse().unwrap();
        let v: i32 = nums.next().unwrap().parse().unwrap();
        edges1.push(vec![u, v]);
    }

    // Input number of edges for tree 2
    let n2: usize = lines.next().unwrap().parse().unwrap();
    let mut edges2: Vec<Vec<i32>> = Vec::new();

    for _ in 0..n2 {
        let line = lines.next().unwrap();
        let mut nums = line.split_whitespace();
        let u: i32 = nums.next().unwrap().parse().unwrap();
        let v: i32 = nums.next().unwrap().parse().unwrap();
        edges2.push(vec![u, v]);
    }

    // Call the solution function
    let result = max_target_nodes(&edges1, &edges2);

    // Output the result
    for i in 0..result.len() {
        print!("{} ", result[i]);
    }
    println!();

    Ok(())
}