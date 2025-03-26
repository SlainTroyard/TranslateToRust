use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        // A min-heap using Reverse wrapper for tuple (nxt, delta, base)
        let mut pq: BinaryHeap<Reverse<(i64, i64, i64)>> = BinaryHeap::new();

        // Initialize the heap with worker times
        for &t in &worker_times {
            let t = t as i64;
            pq.push(Reverse((t, t, t)));
        }

        let mut ans = 0;
        for _ in 0..mountain_height {
            let Reverse((nxt, delta, base)) = pq.pop().unwrap(); // Get the worker with the smallest time
            ans = nxt; // Update the result to the current smallest time
            pq.push(Reverse((nxt + delta + base, delta + base, base))); // Reinsert the worker with updated time
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the mountainHeight and workerTimesSize
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mountain_height = parts[0];
    let worker_times_size = parts[1];

    // Read the workerTimes vector
    let second_line = lines.next().unwrap().unwrap();
    let worker_times: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Check that workerTimesSize matches the actual number of workers provided
    assert_eq!(worker_times_size, worker_times.len() as i32);

    // Solve the problem
    let solution = Solution;
    let result = solution.min_number_of_seconds(mountain_height, worker_times);
    println!("{}", result);
}