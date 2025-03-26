use std::cmp::Reverse;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn results_array(queries: &Vec<Vec<i32>>, k: usize) -> Vec<i32> {
        let mut ans = vec![-1; queries.len()];
        let mut pq = std::collections::BinaryHeap::new();

        for (i, query) in queries.iter().enumerate() {
            let value = query[0].abs() + query[1].abs();
            pq.push(Reverse(value));

            if pq.len() > k {
                pq.pop();
            }

            if pq.len() == k {
                ans[i] = pq.peek().unwrap().0;
            }
        }

        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read queriesSize and k
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let queries_size: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    // Read queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap()?;
        let mut iter = line.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        queries.push(vec![a, b]);
    }

    // Process queries
    let solution = Solution;
    let ans = solution.results_array(&queries, k);

    // Print results
    for num in ans {
        print!("{} ", num);
    }
    println!();

    Ok(())
}