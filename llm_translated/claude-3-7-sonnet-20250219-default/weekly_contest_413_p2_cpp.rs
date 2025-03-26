use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut ans = vec![-1; queries.len()];
        // In Rust, BinaryHeap is a max-heap by default, so we use Reverse to simulate a min-heap
        // This way we can maintain the k largest elements efficiently
        let mut pq = BinaryHeap::new();
        
        for i in 0..queries.len() {
            let manhattan_dist = queries[i][0].abs() + queries[i][1].abs();
            pq.push(Reverse(manhattan_dist));
            
            if pq.len() > k as usize {
                pq.pop();
            }
            
            if pq.len() == k as usize {
                // The smallest of our k largest elements is at the top of our min-heap
                ans[i] = pq.peek().unwrap().0;
            }
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the queries size and k
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let queries_size: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    
    // Read the queries
    let mut queries = vec![vec![0; 2]; queries_size];
    for i in 0..queries_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        queries[i][0] = parts.next().unwrap().parse().unwrap();
        queries[i][1] = parts.next().unwrap().parse().unwrap();
    }
    
    // Solve and print the result
    let ans = Solution::results_array(queries, k);
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
    println!();
}