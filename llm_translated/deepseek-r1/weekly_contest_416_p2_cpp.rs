use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::io::{self, Read};

fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
    // Use a min-heap to track the next available times for each worker
    let mut heap = BinaryHeap::new();
    
    // Initialize the heap with each worker's initial time
    for &base in &worker_times {
        let t = base as i64;
        heap.push(Reverse((t, t, base)));
    }
    
    let mut ans = 0;
    // Process each mountain height decrement
    for _ in 0..mountain_height {
        let Reverse((nxt, delta, base)) = heap.pop().unwrap();
        ans = nxt;
        
        // Calculate new next time and delta for the worker
        let base_i64 = base as i64;
        let new_delta = delta + base_i64;
        let new_nxt = nxt + new_delta;
        heap.push(Reverse((new_nxt, new_delta, base)));
    }
    
    ans
}

fn main() {
    // Read all input at once and split into tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().expect("Invalid number"));
    
    // Parse mountain height and worker times size
    let mountain_height = tokens.next().expect("Missing mountain height");
    let worker_times_size = tokens.next().expect("Missing worker times size");
    
    // Collect worker times from remaining tokens
    let worker_times: Vec<i32> = tokens
        .by_ref()
        .take(worker_times_size as usize)
        .collect();
    
    // Ensure correct number of worker times were provided
    if worker_times.len() != worker_times_size as usize {
        panic!("Insufficient worker times provided");
    }
    
    // Calculate and print the result
    let result = min_number_of_seconds(mountain_height, worker_times);
    println!("{}", result);
}