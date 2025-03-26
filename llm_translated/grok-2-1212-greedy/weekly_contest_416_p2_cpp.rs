use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

// Define a custom type for the priority queue
type PQItem = (i64, i64, i64);
type PQ = BinaryHeap<Reverse<PQItem>>;

struct Solution;

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut pq: PQ = BinaryHeap::new();
        for &t in &worker_times {
            pq.push(Reverse((t as i64, t as i64, t as i64)));
        }
        let mut ans = 0;
        let mut height = mountain_height;
        while height > 0 {
            let Reverse((nxt, delta, base)) = pq.pop().unwrap();
            ans = nxt;
            pq.push(Reverse((nxt + delta + base, delta + base, base)));
            height -= 1;
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read mountain height and worker times size
    let first_line: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mountain_height = first_line[0];
    let worker_times_size = first_line[1];

    // Read worker times
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