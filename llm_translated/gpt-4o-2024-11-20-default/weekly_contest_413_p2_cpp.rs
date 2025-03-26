use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::io;

struct Solution;

impl Solution {
    pub fn results_array(queries: Vec<Vec<i32>>, k: usize) -> Vec<i32> {
        let mut ans = vec![-1; queries.len()];
        let mut pq = BinaryHeap::new();

        for (i, query) in queries.iter().enumerate() {
            let distance = query[0].abs() + query[1].abs();
            pq.push(Reverse(distance));

            // Ensure the priority queue holds only the top k elements
            if pq.len() > k {
                pq.pop();
            }

            // If the heap size equals k, record the k-th largest distance
            if pq.len() == k {
                ans[i] = pq.peek().unwrap().0; // pq.peek().unwrap() gives the smallest item in the BinaryHeap because of Reverse
            }
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    let mut stdin = io::stdin();

    // Reading the number of queries and k
    stdin.read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let queries_size: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Reading the queries
    let mut queries = Vec::new();
    for _ in 0..queries_size {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        parts = input.trim().split_whitespace();
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        queries.push(vec![x, y]);
    }

    // Solve the problem using the Solution struct
    let sol = Solution;
    let ans = sol.results_array(queries, k);

    // Output the result array
    for (i, value) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", value);
    }
    println!();
}