use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of nodes `n` and the threshold
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let threshold: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the edges
    let mut edges: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let edge: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges.push(edge);
    }

    // Create a solution instance and compute the result
    let solution = Solution;
    let result = solution.min_max_weight(n, &edges, threshold);

    // Print the result
    println!("{}", result);
}

struct Solution;

impl Solution {
    fn min_max_weight(&self, n: usize, edges: &Vec<Vec<i32>>, threshold: i32) -> i32 {
        // Define a closure to check if all nodes are reachable with a given weight limit
        let check = |lim: i32| {
            let mut adjacency_list: Vec<Vec<usize>> = vec![Vec::new(); n];
            for edge in edges {
                if edge[2] <= lim {
                    adjacency_list[edge[1] as usize].push(edge[0] as usize);
                }
            }

            let mut visited = vec![false; n];
            let mut queue = VecDeque::new();
            queue.push_back(0);
            visited[0] = true;

            while let Some(sn) = queue.pop_front() {
                for &fn_ in &adjacency_list[sn] {
                    if !visited[fn_] {
                        visited[fn_] = true;
                        queue.push_back(fn_);
                    }
                }
            }

            visited.iter().all(|&v| v)
        };

        // Find the maximum weight in the edges
        let max_weight = edges.iter().map(|edge| edge[2]).max().unwrap_or(0);

        // If the maximum weight doesn't satisfy the condition, return -1
        if !check(max_weight) {
            return -1;
        }

        // Perform binary search to find the minimum maximum weight
        let mut head = 0;
        let mut tail = max_weight;
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