// Problem: Weekly Contest 413 Problem 2
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn results_array(queries: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut ans = vec![-1; queries.len()];
        let mut pq = BinaryHeap::new();
        
        for i in 0..queries.len() {
            // Calculate the sum of absolute values
            pq.push(queries[i][0].abs() + queries[i][1].abs());
            
            // If the priority queue size exceeds k, remove the largest element
            if pq.len() > k as usize {
                pq.pop();
            }
            
            // If the priority queue size is exactly k, the top element is our answer
            if pq.len() == k as usize {
                ans[i] = *pq.peek().unwrap();
            }
        }
        
        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse the first line containing queries_size and k
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let queries_size: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read the queries
    let mut queries = vec![vec![0; 2]; queries_size];
    for i in 0..queries_size {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        queries[i][0] = iter.next().unwrap().parse().unwrap();
        queries[i][1] = iter.next().unwrap().parse().unwrap();
    }
    
    // Solve the problem
    let sol = Solution;
    let ans = sol.results_array(&queries, k);
    
    // Print the result
    for (i, val) in ans.iter().enumerate() {
        print!("{}", val);
        if i < ans.len() - 1 {
            print!(" ");
        }
    }
    println!();
}