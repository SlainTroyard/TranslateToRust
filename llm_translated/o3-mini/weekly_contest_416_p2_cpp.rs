use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead, Write};

// The function that implements the logic from the C++ solution.
// It calculates the minimum number of seconds required based on mountainHeight
// and the workerTimes (converted to i64).
fn min_number_of_seconds(mountain_height: i64, worker_times: &[i64]) -> i64 {
    // The priority queue ordered by the next available time.
    // We'll store tuples of (nxt, delta, base) inside Reverse to simulate a min-heap.
    let mut pq = BinaryHeap::new();
    // Initialize the priority queue with each worker's initial values.
    for &t in worker_times {
        // We convert t into i64
        pq.push(Reverse((t, t, t)));
    }
    
    let mut ans = 0;
    // Process the mountain tasks one by one.
    let mut remaining = mountain_height;
    while remaining > 0 {
        // Pop the worker with the smallest next available time.
        let Reverse((nxt, delta, base)) = pq.pop().expect("Priority queue should not be empty");
        ans = nxt;
        // Compute the next availability for this worker and then push it back in.
        let new_tuple = (nxt + delta + base, delta + base, base);
        pq.push(Reverse(new_tuple));
        remaining -= 1;
    }
    ans
}

fn main() -> io::Result<()> {
    // Set up standard input and output.
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    // We'll read the entire input from stdin. 
    // The original C++ code expects first two numbers: mountainHeight and workerTimesSize.
    // Then it expects workerTimes in subsequent positions.
    let mut input = String::new();
    for line in stdin.lock().lines() {
        let line = line?;
        input.push_str(&line);
        input.push(' '); // add a space to make sure numbers are separated
    }
    
    // Split the input into tokens (whitespace separated).
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() < 2 {
        writeln!(out, "Invalid input")?;
        return Ok(());
    }
    
    // Parse mountainHeight and workerTimesSize from input tokens.
    let mountain_height: i64 = tokens[0].parse().expect("Invalid mountain height");
    let worker_times_size: usize = tokens[1].parse().expect("Invalid worker times size");
    
    // Ensure that we have the right number of workerTimes.
    if tokens.len() < 2 + worker_times_size {
        writeln!(out, "Invalid input: not enough worker times")?;
        return Ok(());
    }
    
    // Parse the workerTimes into a vector of i64.
    let worker_times: Vec<i64> = tokens[2..2 + worker_times_size]
        .iter()
        .map(|&s| s.parse().expect("Invalid worker time"))
        .collect();
    
    // Compute the result using our function.
    let result = min_number_of_seconds(mountain_height, &worker_times);
    
    // Print the result, matching the C++ code's output format.
    writeln!(out, "{}", result)?;
    Ok(())
}