use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: &Vec<i32>) -> i64 {
        // Use a min-heap (BinaryHeap in Rust is a max-heap, so use Reverse to simulate min-heap)
        let mut pq: BinaryHeap<Reverse<(i64, i64, i64)>> = BinaryHeap::new();

        for &t in worker_times {
            pq.push(Reverse((t as i64, t as i64, t as i64))); // (nxt, delta, base)
        }

        let mut ans: i64 = 0;
        for _ in 0..mountain_height {
            let Reverse((nxt, delta, base)) = pq.pop().unwrap();
            ans = nxt;
            pq.push(Reverse((nxt + delta + base, delta + base, base)));
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();

    let mountain_height: i32 = first_line_iter.next().unwrap().parse().unwrap();
    let worker_times_size: i32 = first_line_iter.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap().unwrap();
    let worker_times: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let s = Solution {};
    println!("{}", s.min_number_of_seconds(mountain_height, &worker_times));
}