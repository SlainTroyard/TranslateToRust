use std::collections::BinaryHeap;
use std::io::{self, BufRead, Write};

// This function mimics the logic of the C++ resultsArray method.
// It processes each query, computes the Manhattan distance (abs(x) + abs(y)),
// and maintains a max-heap of the k smallest distances seen so far.
// When the heap's size equals k, the top of the heap is the kth smallest distance.
fn results_array(queries: &[Vec<i32>], k: usize) -> Vec<i32> {
    let mut ans = vec![-1; queries.len()];
    let mut pq = BinaryHeap::new();

    for (i, query) in queries.iter().enumerate() {
        let distance = query[0].abs() + query[1].abs();
        pq.push(distance);
        if pq.len() > k {
            // Remove the largest element so that the heap always contains the k smallest
            pq.pop();
        }
        if pq.len() == k {
            // The top is the kth smallest distance so far
            ans[i] = *pq.peek().unwrap();
        }
    }
    ans
}

fn main() {
    // Use a BufReader for efficient input handling
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    let mut input = String::new();
    // Read the entire input
    reader.read_to_string(&mut input).expect("Failed to read input");
    
    // Split the input by whitespace to mimic C++ >> operator behavior
    let mut tokens = input.split_whitespace();

    // Read the number of queries and k from the input.
    // This follows exactly the same order as provided in the original C++ code.
    let queries_size: usize = tokens
        .next()
        .expect("Missing queries size")
        .parse()
        .expect("Invalid queries size");
    let k: usize = tokens
        .next()
        .expect("Missing k")
        .parse()
        .expect("Invalid value of k");

    // Parse each query (each consisting of 2 integers).
    let mut queries: Vec<Vec<i32>> = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let x: i32 = tokens
            .next()
            .expect("Missing x coordinate")
            .parse()
            .expect("Invalid x coordinate");
        let y: i32 = tokens
            .next()
            .expect("Missing y coordinate")
            .parse()
            .expect("Invalid y coordinate");
        queries.push(vec![x, y]);
    }

    // Compute the answer using the translated function logic.
    let ans = results_array(&queries, k);

    // Write the results, preserving the exact format from the C++ code:
    // each answer followed by a space, ending with a newline.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for value in ans {
        write!(handle, "{} ", value).expect("Failed to write output");
    }
    writeln!(handle).expect("Failed to write newline");
}