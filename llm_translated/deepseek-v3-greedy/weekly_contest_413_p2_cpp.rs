use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    // Read the number of queries and k from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let queries_size: usize = iter.next().unwrap().parse().expect("Invalid input");
    let k: usize = iter.next().unwrap().parse().expect("Invalid input");

    // Read the queries from stdin
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let mut iter = input.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().expect("Invalid input");
        let y: i32 = iter.next().unwrap().parse().expect("Invalid input");
        queries.push(vec![x, y]);
    }

    // Compute the results array
    let ans = results_array(&queries, k);

    // Print the results
    for val in ans {
        print!("{} ", val);
    }
    println!();
}

fn results_array(queries: &Vec<Vec<i32>>, k: usize) -> Vec<i32> {
    let mut ans = vec![-1; queries.len()];
    let mut pq = BinaryHeap::new();

    for (i, query) in queries.iter().enumerate() {
        let sum = query[0].abs() + query[1].abs();
        pq.push(Reverse(sum));
        if pq.len() > k {
            pq.pop();
        }
        if pq.len() == k {
            if let Some(&Reverse(top)) = pq.peek() {
                ans[i] = top;
            }
        }
    }

    ans
}