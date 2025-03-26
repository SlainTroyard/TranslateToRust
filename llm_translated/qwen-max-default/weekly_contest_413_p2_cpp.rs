use std::io::{self, BufRead, Write};

fn main() {
    // Read the number of queries and k from stdin
    let (queries_size, k) = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let nums: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();
        (nums[0], nums[1])
    };

    // Read the queries from stdin
    let mut queries = vec![(0, 0); queries_size];
    for i in 0..queries_size {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let nums: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();
        queries[i] = (nums[0], nums[1]);
    }

    // Solve the problem
    let ans = results_array(&queries, k);

    // Output the results
    for &result in &ans {
        print!("{} ", result);
    }
    println!();
}

/// Function to solve the problem
fn results_array(queries: &[(i32, i32)], k: usize) -> Vec<i32> {
    let mut ans = vec![-1; queries.len()];
    let mut pq = std::collections::BinaryHeap::new();

    for (i, &(x, y)) in queries.iter().enumerate() {
        pq.push((x.abs() + y.abs()));
        if pq.len() > k {
            pq.pop();
        }
        if pq.len() == k {
            ans[i] = *pq.peek().unwrap();
        }
    }

    ans
}