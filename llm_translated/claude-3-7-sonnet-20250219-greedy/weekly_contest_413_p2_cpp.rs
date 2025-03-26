use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn results_array(queries: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut ans = vec![-1; queries.len()];
        // In Rust, priority_queue is a max-heap by default
        // To simulate C++'s behavior, we use Reverse to make it a min-heap
        // and then negate the values to effectively get a max-heap
        let mut pq = BinaryHeap::new();
        
        for i in 0..queries.len() {
            let manhattan_dist = queries[i][0].abs() + queries[i][1].abs();
            pq.push(manhattan_dist);
            
            if pq.len() > k as usize {
                pq.pop();
            }
            
            if pq.len() == k as usize {
                ans[i] = *pq.peek().unwrap();
            }
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read queries_size and k
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let queries_size: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read queries
    let mut queries = vec![vec![0; 2]; queries_size];
    for i in 0..queries_size {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        queries[i][0] = iter.next().unwrap().parse().unwrap();
        queries[i][1] = iter.next().unwrap().parse().unwrap();
    }
    
    // Solve and output
    let ans = Solution::results_array(&queries, k);
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
    println!();
}