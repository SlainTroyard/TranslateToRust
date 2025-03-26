use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn results_array(queries: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut ans = vec![-1; queries.len()];
        let mut pq = BinaryHeap::new();
        
        for (i, query) in queries.iter().enumerate() {
            pq.push(Reverse(query[0].abs() + query[1].abs()));
            if pq.len() > k as usize {
                pq.pop();
            }
            if pq.len() == k as usize {
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
    let mut numbers = first_line.split_whitespace();
    let queries_size: usize = numbers.next().unwrap().parse().unwrap();
    let k: i32 = numbers.next().unwrap().parse().unwrap();

    // Read queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap()?;
        let mut numbers = line.split_whitespace();
        let a: i32 = numbers.next().unwrap().parse().unwrap();
        let b: i32 = numbers.next().unwrap().parse().unwrap();
        queries.push(vec![a, b]);
    }

    // Process queries and get results
    let sol = Solution;
    let ans = sol.results_array(&queries, k);

    // Print results
    for num in ans {
        print!("{} ", num);
    }
    println!();

    Ok(())
}