use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

// Define a struct to represent the worker's time information
struct WorkerTime {
    next: i64,
    delta: i64,
    base: i64,
}

impl Ord for WorkerTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.next.cmp(&other.next)
    }
}

impl PartialOrd for WorkerTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for WorkerTime {
    fn eq(&self, other: &Self) -> bool {
        self.next == other.next
    }
}

impl Eq for WorkerTime {}

struct Solution;

impl Solution {
    fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        // Create a min-heap to store worker times
        let mut pq = BinaryHeap::new();
        for &t in &worker_times {
            pq.push(Reverse(WorkerTime {
                next: t as i64,
                delta: t as i64,
                base: t as i64,
            }));
        }

        let mut ans = 0;
        for _ in 0..mountain_height {
            // Get the worker with the earliest next available time
            let Reverse(WorkerTime { next, delta, base }) = pq.pop().unwrap();
            ans = next;

            // Update the worker's next available time and push back to the heap
            pq.push(Reverse(WorkerTime {
                next: next + delta + base,
                delta: delta + base,
                base,
            }));
        }

        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input values
    let first_line: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mountain_height = first_line[0];
    let worker_times_size = first_line[1];

    let worker_times: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print the result
    let solution = Solution;
    let result = solution.min_number_of_seconds(mountain_height, worker_times);
    println!("{}", result);

    Ok(())
}