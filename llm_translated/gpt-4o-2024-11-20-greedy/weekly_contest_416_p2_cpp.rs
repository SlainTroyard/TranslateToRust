use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        // Priority queue to store tuples (next_time, delta, base_time)
        let mut pq = BinaryHeap::new();
        
        // Initialize the priority queue with workers' times
        for &t in &worker_times {
            pq.push(Reverse((t as i64, t as i64, t as i64)));
        }

        let mut ans = 0;
        for _ in 0..mountain_height {
            // Extract the worker with the smallest next_time
            let Reverse((nxt, delta, base)) = pq.pop().unwrap();
            ans = nxt;
            // Push the updated tuple back into the priority queue
            pq.push(Reverse((nxt + delta + base, delta + base, base)));
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse mountainHeight and workerTimesSize
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let mountain_height: i32 = first_line_iter.next().unwrap().parse().unwrap();
    let worker_times_size: usize = first_line_iter.next().unwrap().parse().unwrap();

    // Parse workerTimes
    let second_line = lines.next().unwrap().unwrap();
    let worker_times: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure the input size matches the expected size
    assert_eq!(worker_times.len(), worker_times_size);

    // Solve the problem
    let solution = Solution;
    let result = solution.min_number_of_seconds(mountain_height, worker_times);

    // Print the result
    println!("{}", result);
}