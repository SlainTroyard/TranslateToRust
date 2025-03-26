use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::io;
use std::io::BufRead;

fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> u64 {
    let mut pq = BinaryHeap::new();
    for &t in &worker_times {
        pq.push(Reverse((t as u64, t as u64, t as u64))); // (nxt, delta, base) as u64 for calculations
    }

    let mut ans = 0;
    let mut current_height = mountain_height;
    while current_height > 0 {
        if let Some(Reverse((nxt, delta, base))) = pq.pop() {
            ans = nxt;
            pq.push(Reverse((nxt + delta + base, delta + base, base)));
            current_height -= 1;
        } else {
            break; // Should not happen if input is valid as per problem description
        }
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().expect("Failed to read line");
    let mut first_line_iter = first_line.split_whitespace();
    let mountain_height: i32 = first_line_iter.next().unwrap().parse().expect("Invalid mountainHeight");
    let worker_times_size: usize = first_line_iter.next().unwrap().parse().expect("Invalid workerTimesSize");

    let second_line = lines.next().unwrap().expect("Failed to read line");
    let worker_times: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid workerTime"))
        .collect();

    if worker_times.len() != worker_times_size {
        eprintln!("Warning: workerTimesSize does not match the number of worker times provided.");
    }

    let result = min_number_of_seconds(mountain_height, worker_times);
    println!("{}", result);
}