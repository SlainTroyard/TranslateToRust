// Problem: Weekly Contest 413 Problem 2
use std::io::{self, BufRead};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    // Read the first line of input: queriesSize and k
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let queries_size: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Read the queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let query_line = lines.next().unwrap().unwrap();
        let mut query_parts = query_line.split_whitespace();
        let x: i32 = query_parts.next().unwrap().parse().unwrap();
        let y: i32 = query_parts.next().unwrap().parse().unwrap();
        queries.push((x, y));
    }

    // Call the results_array function
    let result = results_array(&queries, k);

    // Print the result
    for val in result {
        print!("{} ", val);
    }
}

/// Computes the Manhattan distance of a point (x, y) from the origin.
fn manhattan_distance(x: i32, y: i32) -> i32 {
    x.abs() + y.abs()
}

/// Implements the resultsArray function in idiomatic Rust.
fn results_array(queries: &[(i32, i32)], k: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(queries.len());
    let mut heap = BinaryHeap::new(); // Max-heap using Reverse to simulate min-heap behavior

    for &(x, y) in queries {
        let distance = manhattan_distance(x, y);

        // Insert the distance into the heap
        heap.push(Reverse(distance));

        // If the heap exceeds size k, remove the largest element
        if heap.len() > k {
            heap.pop();
        }

        // If the heap has exactly k elements, the top is the k-th largest
        if heap.len() == k {
            result.push(heap.peek().unwrap().0);
        } else {
            result.push(-1); // Not enough elements in the heap
        }
    }

    result
}