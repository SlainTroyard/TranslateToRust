use std::io::{self, BufRead, Write};
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn results_array(queries: &Vec<Vec<i32>>, k: usize) -> Vec<i32> {
        let mut ans = vec![-1; queries.len()];
        let mut pq = BinaryHeap::new();
        
        for (i, query) in queries.iter().enumerate() {
            let value = (query[0].abs() + query[1].abs()) as i32;
            pq.push(value);
            
            if pq.len() > k {
                pq.pop();
            }
            
            if pq.len() == k {
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

    // Parse the first line to get the size of queries and k
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let queries_size: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    // Initialize the queries vector
    let mut queries = vec![vec![0; 2]; queries_size];

    // Parse the remaining lines to fill the queries vector
    for i in 0..queries_size {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        queries[i][0] = iter.next().unwrap().parse().unwrap();
        queries[i][1] = iter.next().unwrap().parse().unwrap();
    }

    // Solve the problem
    let sol = Solution;
    let ans = sol.results_array(&queries, k);

    // Write the result to stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for (i, &value) in ans.iter().enumerate() {
        write!(handle, "{}", value).unwrap();
        if i < ans.len() - 1 {
            write!(handle, " ").unwrap();
        }
    }
    writeln!(handle).unwrap();
}