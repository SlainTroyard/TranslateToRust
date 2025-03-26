use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn results_array(queries: Vec<Vec<i32>>, k: usize) -> Vec<i32> {
        let mut ans = vec![-1; queries.len()];
        let mut pq = BinaryHeap::new();
        
        for (i, query) in queries.iter().enumerate() {
            // Calculate the Manhattan distance
            let distance = query[0].abs() + query[1].abs();
            // Push it into the max-heap (using Reverse to simulate a min-heap in Rust)
            pq.push(Reverse(distance));
            
            // If heap size exceeds k, remove the smallest element
            if pq.len() > k {
                pq.pop();
            }

            // If the heap size equals k, record the top of the heap
            if pq.len() == k {
                // `pq.peek()` returns an Option, so we unwrap here since size == k guarantees it's not empty
                if let Some(Reverse(top)) = pq.peek() {
                    ans[i] = *top;
                }
            }
        }
        ans
    }
}

fn main() {
    // Read the input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // First line contains the size of queries and k
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let queries_size: usize = first_line_iter.next().unwrap().parse().unwrap();
    let k: usize = first_line_iter.next().unwrap().parse().unwrap();
    
    // The next `queries_size` lines contain the individual queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        queries.push(vec![nums.next().unwrap(), nums.next().unwrap()]);
    }
    
    // Solve the problem
    let sol = Solution;
    let ans = sol.results_array(queries, k);
    
    // Print the result
    let mut output = String::new();
    for value in ans {
        output.push_str(&format!("{} ", value));
    }
    // Remove the trailing space and print the output
    output.pop();
    println!("{}", output);
}