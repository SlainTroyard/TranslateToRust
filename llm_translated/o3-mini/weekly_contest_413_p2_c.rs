use std::error::Error;
use std::io::{self, Read};

/// This function processes the queries according to the problem logic.
/// For each query coordinate, it computes its Manhattan distance and
/// then maintains a descending-sorted array of the largest k distances seen so far.
/// Once k distances have been processed, the kth largest (which is the smallest among the k largest)
/// is stored in the result for that query index. Before k distances are processed, the result remains -1.
fn results_array(queries: &[[i32; 2]], k: usize) -> Vec<i32> {
    let queries_size = queries.len();
    // Preinitialize the result array with -1 for each query.
    let mut result = vec![-1; queries_size];

    // distance_arr will hold the top k distances, sorted in descending order.
    // Here we create a vector with exactly k elements (initial values don't matter until assigned).
    let mut distance_arr = vec![0; k];
    // Represents the current number of valid distances in distance_arr.
    let mut distance_size = 0;

    // Iterate over each query.
    for (i, query) in queries.iter().enumerate() {
        // Compute Manhattan distance
        let distance = query[0].abs() + query[1].abs();

        // j is used for shifting elements to maintain a sorted order (descending)
        // starting from the current size.
        let mut j = distance_size;
        // Shift all smaller distances up to free space for the new one.
        while j > 0 && distance_arr[j - 1] < distance {
            if j < k {
                distance_arr[j] = distance_arr[j - 1]; // shift element rightward
            }
            j -= 1;
        }
        // If the insertion position is within k, insert the new distance.
        if j < k {
            distance_arr[j] = distance;
            if distance_size < k {
                distance_size += 1;
            }
        }

        // Once we have k distances, the kth largest (i.e. the smallest in the top k) is at index k-1.
        if distance_size == k {
            result[i] = distance_arr[k - 1];
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read entire input from stdin into a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split input by whitespace and parse numbers.
    let mut tokens = input.split_whitespace();

    // The first two numbers are the number of queries and k, respectively.
    let queries_size: usize = tokens
        .next()
        .ok_or("Expected queries_size")?
        .parse()?;
    let k: usize = tokens
        .next()
        .ok_or("Expected k")?
        .parse()?;

    // Read the query pairs.
    // Each query has 2 integers.
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let a: i32 = tokens
            .next()
            .ok_or("Expected query coordinate")?
            .parse()?;
        let b: i32 = tokens
            .next()
            .ok_or("Expected query coordinate")?
            .parse()?;
        queries.push([a, b]);
    }

    // Compute the results array using the provided queries.
    let result = results_array(&queries, k);

    // Print out the result on stdout.
    // The original C code prints each element separated by a space.
    // We maintain the exact same output format.
    for value in result {
        print!("{} ", value);
    }
    // Ensure the output is flushed.
    Ok(())
}