use std::collections::{VecDeque, HashSet};
use std::cmp::max;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn min_max_weight(n: usize, edges: Vec<Vec<i32>>, threshold: i32) -> i32 {
        // Closure to check if a given weight limit `lim` is feasible
        let check = |lim: i32| -> bool {
            let mut adjacency_list = vec![Vec::new(); n];
            for edge in &edges {
                if edge[2] <= lim {
                    adjacency_list[edge[1] as usize].push(edge[0] as usize);
                }
            }

            let mut visited = vec![false; n];
            let mut queue = VecDeque::new();
            queue.push_back(0);
            visited[0] = true;

            while let Some(node) = queue.pop_front() {
                for &neighbour in &adjacency_list[node] {
                    if !visited[neighbour] {
                        queue.push_back(neighbour);
                        visited[neighbour] = true;
                    }
                }
            }

            // Check if all nodes are visited
            visited.iter().all(|&v| v)
        };

        // Get the maximum weight in edges
        let mut mx = 0;
        for edge in &edges {
            mx = max(mx, edge[2]);
        }

        // If max weight does not satisfy connectivity, return -1
        if !check(mx) {
            return -1;
        }

        // Binary search to find the minimum feasible max weight
        let mut head = 0;
        let mut tail = mx;
        while head < tail {
            let mid = (head + tail) / 2;
            if check(mid) {
                tail = mid;
            } else {
                head = mid + 1;
            }
        }

        head
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of nodes
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the threshold (not used in the algorithm)
    let threshold: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the edges
    let mut edges = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let edge: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        edges.push(edge);
    }

    // Solve the problem
    let solution = Solution;
    println!("{}", solution.min_max_weight(n, edges, threshold));
}