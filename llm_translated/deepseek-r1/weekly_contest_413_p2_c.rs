// Problem: Weekly Contest 413 Problem 2
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    // Read all input at once for efficient parsing
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    
    // Parse number of queries and k value
    let queries_size = tokens.next().unwrap().parse::<usize>().unwrap();
    let k = tokens.next().unwrap().parse::<usize>().unwrap();
    
    // Read all query points
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let x = tokens.next().unwrap().parse::<i32>().unwrap();
        let y = tokens.next().unwrap().parse::<i32>().unwrap();
        queries.push((x, y));
    }
    
    // Process queries and get results
    let result = results_array(&queries, k);
    
    // Print results in exact original format with trailing space
    for res in result {
        print!("{} ", res);
    }
    println!();
}

fn results_array(queries: &[(i32, i32)], k: usize) -> Vec<i32> {
    let mut result = vec![-1; queries.len()];
    let mut heap = BinaryHeap::with_capacity(k + 1); // Slightly larger to prevent reallocations
    
    for (i, &(x, y)) in queries.iter().enumerate() {
        // Calculate Manhattan distance (same as original code)
        let distance = x.abs() + y.abs();
        
        // Maintain max-heap with up to k smallest elements
        heap.push(distance);
        
        // Remove largest element if we exceed k elements
        if heap.len() > k {
            heap.pop(); // This removes the maximum value (heap property)
        }
        
        // Store current k-th smallest or -1 if not enough elements
        result[i] = if heap.len() == k {
            *heap.peek().unwrap()
        } else {
            -1
        };
    }
    
    result
}