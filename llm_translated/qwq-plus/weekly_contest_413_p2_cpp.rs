use std::collections::BinaryHeap;
use std::io;
use std::io::prelude::*;

struct Solution;

impl Solution {
    /// Computes the array where each element is the k-th smallest Manhattan distance up to that query.
    /// Uses a max-heap to track the k smallest distances.
    fn results_array(queries: &Vec<(i32, i32)>, k: usize) -> Vec<i32> {
        let n = queries.len();
        let mut ans = vec![-1; n];
        let mut heap = BinaryHeap::new();
        for (i, &(x, y)) in queries.iter().enumerate() {
            let distance = x.abs() + y.abs();
            heap.push(distance);
            if heap.len() > k {
                heap.pop();
            }
            if heap.len() == k {
                ans[i] = *heap.peek().unwrap();
            }
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line for queries size and k
    let first_line = lines.next().expect("No first line").unwrap();
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    let queries_size: usize = parts[0].parse().unwrap();
    let k: usize = parts[1].parse().unwrap();

    // Read the queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().expect("Not enough queries").unwrap();
        let coords: Vec<&str> = line.split_whitespace().collect();
        let x: i32 = coords[0].parse().unwrap();
        let y: i32 = coords[1].parse().unwrap();
        queries.push((x, y));
    }

    // Compute the result and print
    let ans = Solution::results_array(&queries, k);
    for &num in &ans {
        print!("{} ", num);
    }
    println!();
}