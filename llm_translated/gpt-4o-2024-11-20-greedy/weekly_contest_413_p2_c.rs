use std::io::{self, BufRead};
use std::cmp::Ordering;

fn results_array(queries: Vec<(i32, i32)>, k: usize) -> Vec<i32> {
    let mut result = vec![-1; queries.len()];
    let mut distance_arr = Vec::with_capacity(k);

    for (i, &(x, y)) in queries.iter().enumerate() {
        let distance = x.abs() + y.abs();

        // Insert the distance into the sorted array
        let mut j = distance_arr.len();
        while j > 0 && distance_arr[j - 1] < distance {
            if j < k {
                if j == distance_arr.len() {
                    distance_arr.push(distance_arr[j - 1]);
                } else {
                    distance_arr[j] = distance_arr[j - 1];
                }
            }
            j -= 1;
        }
        if j < k {
            if j == distance_arr.len() {
                distance_arr.push(distance);
            } else {
                distance_arr[j] = distance;
            }
        }

        // If we have k distances, set the result for this query
        if distance_arr.len() == k {
            result[i] = distance_arr[k - 1];
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read queriesSize and k
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let queries_size: usize = first_line_iter.next().unwrap().parse().unwrap();
    let k: usize = first_line_iter.next().unwrap().parse().unwrap();

    // Read queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        queries.push((x, y));
    }

    // Compute results
    let result = results_array(queries, k);

    // Print results
    for value in result {
        print!("{} ", value);
    }
    println!();
}