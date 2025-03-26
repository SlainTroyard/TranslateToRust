use std::collections::BinaryHeap;
use std::io;
use std::str::SplitWhitespace;

struct Solution {}

impl Solution {
    pub fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let n = queries.len();
        let mut ans = vec![-1; n]; // Initialize the result vector with -1
        let mut pq = BinaryHeap::new(); // Use BinaryHeap as a max-heap

        for i in 0..n {
            // Calculate Manhattan distance for the current query
            let dist = queries[i][0].abs() + queries[i][1].abs();
            pq.push(dist); // Push the distance to the priority queue

            // Maintain the size of priority queue to be at most k
            if pq.len() as i32 > k {
                pq.pop(); // Remove the largest element (top of max-heap)
            }

            // If the priority queue size is k, the top element is the kth largest distance so far
            if pq.len() as i32 == k {
                ans[i] = pq.peek().unwrap(); // Store the top element as the result
            }
        }
        ans // Return the result vector
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Read the first line (queriesSize k)
    let mut iter = input.split_whitespace();

    // Parse queriesSize and k from the first line
    let queries_size: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    // Initialize queries vector
    let mut queries: Vec<Vec<i32>> = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        input.clear();
        io::stdin().read_line(&mut input).unwrap(); // Read each query (x y)
        let mut iter = input.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        queries.push(vec![x, y]); // Store the query in the queries vector
    }

    // Create a Solution instance and call results_array method
    let sol = Solution {};
    let ans = sol.results_array(queries, k);

    // Print the results to stdout, separated by spaces
    for val in ans {
        print!("{} ", val);
    }
    println!(); // Add a newline at the end of the output
}