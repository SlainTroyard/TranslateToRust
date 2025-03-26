// Problem: Weekly Contest 416 Problem 2

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

// A small helper struct to read space-separated input tokens from stdin
// in a way similar to C++'s cin >> operator.
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    // Read and return the next token from stdin,
    // replenishing the buffer as necessary.
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            // If there's any token in the buffer, pop and parse it.
            if let Some(token) = self.buffer.pop() {
                return token.parse().expect("Failed to parse token.");
            }
            // Otherwise, read a new line and split it into tokens.
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("Failed to read line.");
            self.buffer = line
                .split_whitespace()
                .rev() // Reverse so we can pop from the end
                .map(String::from)
                .collect();
        }
    }
}

// Translated from the C++ Solution class to a standalone Rust function.
fn min_number_of_seconds(mountain_height: i32, worker_times: &[i32]) -> i64 {
    // We use a BinaryHeap and store tuples so the smallest 'nxt' is at the top.
    // Rust's BinaryHeap is a max-heap by default, so we wrap 'nxt' in Reverse
    // to turn it into a min-heap based on 'nxt'.
    let mut pq = BinaryHeap::new();

    // Push tuples (Reverse(nxt), nxt, delta, base) for each worker time.
    // In the original C++ code, these were (nxt, delta, base).
    for &t in worker_times {
        let t_i64 = t as i64;
        pq.push((Reverse(t_i64), t_i64, t_i64, t_i64));
    }

    let mut ans = 0i64;

    // Each iteration, pop the smallest 'nxt', update ans,
    // then push a new tuple with updated values.
    for _ in 0..(mountain_height as usize) {
        let (Reverse(_), nxt, delta, base) = pq.pop().expect("Heap unexpectedly empty.");
        ans = nxt;
        let next_nxt = nxt + delta + base;
        let next_delta = delta + base;
        pq.push((Reverse(next_nxt), next_nxt, next_delta, base));
    }

    ans
}

fn main() {
    // Create a Scanner similar to the usage of cin in C++
    let mut scanner = Scanner::new();

    // Read the first two integers: mountainHeight and workerTimesSize
    let mountain_height: i32 = scanner.next();
    let worker_times_size: i32 = scanner.next();

    // Read 'worker_times_size' more integers for the workerTimes array
    let mut worker_times = Vec::with_capacity(worker_times_size as usize);
    for _ in 0..worker_times_size {
        worker_times.push(scanner.next());
    }

    // Compute the answer using the translated function
    let answer = min_number_of_seconds(mountain_height, &worker_times);

    // Print the result to stdout, matching "cout << ... << endl;"
    println!("{}", answer);
}