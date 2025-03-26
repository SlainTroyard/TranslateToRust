use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    // Read the mountain height and the number of worker times
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let mountain_height: i32 = iter.next().expect("No mountain height").parse().expect("Invalid mountain height");
    let worker_times_size: usize = iter.next().expect("No worker times size").parse().expect("Invalid worker times size");

    // Read the worker times
    let mut worker_times = Vec::with_capacity(worker_times_size);
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    for s in input.split_whitespace() {
        worker_times.push(s.parse().expect("Invalid worker time"));
    }

    // Create a Solution instance and compute the result
    let result = Solution::min_number_of_seconds(mountain_height, &worker_times);

    // Print the result
    println!("{}", result);
}

struct Solution;

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: &[i32]) -> i64 {
        // Use a min-heap to keep track of the next available time for each worker
        let mut pq = BinaryHeap::new();
        for &t in worker_times {
            pq.push(Reverse((t as i64, t as i64, t as i64)));
        }

        let mut ans = 0;
        let mut remaining_height = mountain_height;

        while remaining_height > 0 {
            if let Some(Reverse((nxt, delta, base))) = pq.pop() {
                ans = nxt;
                pq.push(Reverse((nxt + delta + base, delta + base, base)));
            }
            remaining_height -= 1;
        }

        ans
    }
}