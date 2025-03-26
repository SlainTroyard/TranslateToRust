use std::io;
use std::io::Read;
use std::collections::VecDeque;

// This function implements the problem logic.
// It returns the minimum maximum weight (lim) such that when using only edges with weight <= lim,
// all vertices can be reached from vertex 0 using a reverse-edge BFS.
fn min_max_weight(n: usize, edges: &Vec<Vec<i32>>, _threshold: i32) -> i32 {
    // Define a closure to check if the graph is connected when using only edges with weight <= lim.
    let check = |lim: i32| -> bool {
        // Create an adjacency list for the graph: For each vertex, collect all vertices that have an 
        // edge leading to it (only if the edge's weight is <= lim). This is the same as doing:
        // for each edge { if (edge[2] <= lim) { graph[edge[1]].push(edge[0]) } } in C++.
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
        for edge in edges {
            // Each edge is represented as [from, to, weight]
            if edge[2] <= lim {
                // Push-back the "from" vertex into the list for the "to" vertex.
                let to = edge[1] as usize;
                let from = edge[0] as usize;
                graph[to].push(from);
            }
        }
        // BFS starting from vertex 0.
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        visited[0] = true;
        queue.push_back(0);
        while let Some(cur) = queue.pop_front() {
            for &neighbor in &graph[cur] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }
        // Return true if all vertices were visited.
        visited.into_iter().all(|v| v)
    };

    // Determine the maximum weight among all edges.
    let mut mx = 0;
    for edge in edges {
        if edge[2] > mx {
            mx = edge[2];
        }
    }

    // If even using all edges (with weight <= mx) the graph is not connected, return -1.
    if !check(mx) {
        return -1;
    }

    // Binary search to find the smallest limit that still makes the graph connected.
    let (mut head, mut tail) = (0, mx);
    while head < tail {
        let mid = (head + tail) >> 1;
        if check(mid) {
            tail = mid;
        } else {
            head = mid + 1;
        }
    }
    head
}

fn main() -> io::Result<()> {
    // Read entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut tokens = input.split_whitespace();

    // Read the number of vertices (n) and the threshold value.
    // The threshold is part of the input even though it is not used in the logic.
    let n: usize = tokens
        .next()
        .expect("Expected n")
        .parse()
        .expect("n should be a number");
    let threshold: i32 = tokens
        .next()
        .expect("Expected threshold")
        .parse()
        .expect("threshold should be a number");

    // Read exactly n edges, each consisting of 3 integers.
    let mut edges: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut edge = Vec::with_capacity(3);
        for _ in 0..3 {
            let num: i32 = tokens
                .next()
                .expect("Expected edge value")
                .parse()
                .expect("Edge value should be a number");
            edge.push(num);
        }
        edges.push(edge);
    }

    // Call the solution function and print the result.
    let result = min_max_weight(n, &edges, threshold);
    println!("{}", result);

    Ok(())
}