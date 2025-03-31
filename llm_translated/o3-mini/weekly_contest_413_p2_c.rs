// Problem: Weekly Contest 413 Problem 2
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    // Read the entire input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line: queriesSize and k
    // The input format is: one line with two integers
    let first_line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing input"))??;
    let mut first_line_iter = first_line.split_whitespace();
    let queries_size: usize = first_line_iter
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing queries size"))?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let k: usize = first_line_iter
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing k value"))?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Read queries_size lines of queries.
    // Each query is two integers on a single line.
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Not enough query lines"))??;
        let mut parts = line.split_whitespace();
        let x: i32 = parts
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing x value"))?
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let y: i32 = parts
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing y value"))?
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        queries.push((x, y));
    }

    // This vector will hold the results for each query.
    // If there are less than k elements so far, result is -1.
    // Otherwise, it is the k-th smallest distance (represented as the maximum in a heap of size k).
    let mut results = Vec::with_capacity(queries_size);

    // Use a max-heap (BinaryHeap in Rust is a max-heap by default)
    // We want to keep track of the k smallest distances seen so far.
    let mut heap = BinaryHeap::with_capacity(k + 1);

    // Process each query
    for (i, (x, y)) in queries.into_iter().enumerate() {
        // Compute the Manhattan distance (absolute values sum)
        let distance = x.abs() + y.abs();

        // Insert the current distance into the max-heap.
        // Since BinaryHeap in Rust is a max-heap, the largest of the k smallest distances will be at the top.
        heap.push(distance);

        // If the heap size exceeds k, remove the largest element.
        if heap.len() > k {
            heap.pop();
        }

        // If we have at least k elements, the top of the heap is the k-th smallest distance seen so far.
        if heap.len() == k {
            // Peek returns an Option<&T>, unwrap is safe because length == k > 0.
            results.push(*heap.peek().unwrap());
        } else {
            results.push(-1);
        }
    }

    // Print results, space separated.
    // This exactly matches the original C code's output format.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for result in results {
        write!(handle, "{} ", result)?;
    }
    // Flush the output to ensure it appears immediately.
    handle.flush()?;
    Ok(())
}