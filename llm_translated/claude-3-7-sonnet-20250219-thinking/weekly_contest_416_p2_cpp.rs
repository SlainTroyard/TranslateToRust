use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, Read};

// MinHeapEntry is a custom struct for priority queue to implement a min-heap
// (BinaryHeap in Rust is a max-heap by default)
#[derive(PartialEq, Eq)]
struct MinHeapEntry(i64, i64, i32);

impl PartialOrd for MinHeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse the comparison to create a min-heap
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinHeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the comparison to create a min-heap
        other.0.cmp(&self.0)
    }
}

struct Solution;

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        // Create a min-heap priority queue
        let mut pq = BinaryHeap::new();
        
        // Initialize the priority queue with worker data
        // Each entry contains: (next time, delta, base)
        for t in worker_times {
            let t_i64 = t as i64;
            pq.push(MinHeapEntry(t_i64, t_i64, t));
        }
        
        let mut ans = 0;
        let mut remaining_height = mountain_height;
        
        // Simulate each step of mountain climbing
        while remaining_height > 0 {
            // Get the worker who will finish next
            let MinHeapEntry(nxt, delta, base) = pq.pop().unwrap();
            ans = nxt;
            
            // Schedule this worker for their next task with increased delta
            pq.push(MinHeapEntry(nxt + delta + base as i64, delta + base as i64, base));
            remaining_height -= 1;
        }
        
        ans
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    
    // Read mountainHeight and workerTimesSize
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a valid number"))
        .collect();
    
    let mountain_height = parts[0];
    let worker_times_size = parts[1] as usize;
    
    // Read workerTimes
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let worker_times: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a valid number"))
        .collect();
    
    // Solve and output the result
    let result = Solution::min_number_of_seconds(mountain_height, worker_times);
    println!("{}", result);
}