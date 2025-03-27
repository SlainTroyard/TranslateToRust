// Problem: Weekly Contest 413 Problem 2
use std::io;
use std::collections::BinaryHeap;

// Implement resultsArray function
fn results_array(queries: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity(queries.len());
    
    // Using BinaryHeap as a max heap
    let mut heap = BinaryHeap::new();
    
    for query in queries {
        // Calculate Manhattan distance
        let distance = query[0].abs() + query[1].abs();
        
        // Insert the current distance into the heap
        heap.push(distance);
        
        // If the heap size exceeds k, remove the largest element
        if heap.len() > k as usize {
            heap.pop();
        }
        
        // If the heap size equals k, return the top element (k-th smallest distance)
        if heap.len() == k as usize {
            result.push(*heap.peek().unwrap());
        } else {
            result.push(-1);  // Heap has fewer than k elements
        }
    }
    
    result
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    // Parse queriesSize and k
    let parts: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    let queries_size = parts[0] as usize;
    let k = parts[1];
    
    // Read queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        let coords: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        
        queries.push(vec![coords[0], coords[1]]);
    }
    
    // Process queries and get results
    let result = results_array(&queries, k);
    
    // Print results with spaces in between
    for (i, val) in result.iter().enumerate() {
        print!("{}", val);
        if i < result.len() - 1 {
            print!(" ");
        }
    }
    
    Ok(())
}