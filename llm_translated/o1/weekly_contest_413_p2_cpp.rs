// Weekly Contest 413 Problem 2 in Rust
// Translated from the provided C++ solution.
// This solution reads input from stdin and writes the result to stdout
// in the EXACT SAME format as the original C++ code.

use std::io::{self, BufRead};
use std::collections::BinaryHeap;

/// Represents our solution, analogous to the C++ `Solution` class.
struct Solution;

impl Solution {
    /// Translates the C++ `resultsArray` function.
    /// Given queries of coordinates (x, y) and a parameter k,
    /// it maintains a max-heap of up to k many absolute-sums of coordinates.
    /// Once the heap is full (size == k), we record the top of that heap.
    fn results_array(&self, queries: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        // Initialize the results with -1, analogous to the C++:
        // vector<int> ans(queries.size(), -1);
        let mut ans = vec![-1; queries.len()];
        
        // In Rust, BinaryHeap is a max-heap by default, matching C++'s priority_queue<int>.
        let mut pq = BinaryHeap::new();
        
        for (i, query) in queries.iter().enumerate() {
            // The absolute sum of the coordinates:
            let sum = query[0].abs() + query[1].abs();
            pq.push(sum);
            
            // Pop if we exceed size k
            if pq.len() as i32 > k {
                pq.pop();
            }
            
            // If we have exactly k elements in the heap, record the top
            if pq.len() as i32 == k {
                ans[i] = *pq.peek().unwrap();
            }
        }
        
        ans
    }
}

fn main() {
    // Prepare to read from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // STEP 1: Read queriesSize and k
    // This closely matches the logic:
    //   int queriesSize, k;
    //   cin >> queriesSize >> k;
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let queries_size: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    
    // STEP 2: Read the queries array (each query has two integers)
    //   vector<vector<int>> queries(queriesSize, vector<int>(2));
    //   for (int i = 0; i < queriesSize; i++) {
    //       cin >> queries[i][0] >> queries[i][1];
    //   }
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap().unwrap();
        let mut it = line.split_whitespace();
        let x: i32 = it.next().unwrap().parse().unwrap();
        let y: i32 = it.next().unwrap().parse().unwrap();
        queries.push(vec![x, y]);
    }
    
    // Instantiate our solution and compute the results
    let sol = Solution;
    let ans = sol.results_array(&queries, k);
    
    // STEP 3: Output the results in the exact same format:
    //   for (int i = 0; i < ans.size(); i++) {
    //       cout << ans[i] << " ";
    //   }
    //   cout << endl;
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
    println!();
}