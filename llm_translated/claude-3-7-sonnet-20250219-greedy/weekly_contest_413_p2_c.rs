use std::io::{self, BufRead};

/// Processes queries and returns an array of results based on the k-th largest Manhattan distance.
///
/// # Arguments
/// * `queries` - A 2D vector where each inner vector contains [x, y] coordinates
/// * `k` - The k-th largest Manhattan distance to track
///
/// # Returns
/// A vector of results where each element is the k-th largest Manhattan distance seen so far,
/// or -1 if fewer than k distances have been processed.
fn results_array(queries: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let queries_size = queries.len();
    let mut result = vec![-1; queries_size];
    
    let mut distance_arr = vec![0; (k + 1) as usize];
    let mut distance_size = 0;
    
    for i in 0..queries_size {
        let distance = queries[i][0].abs() + queries[i][1].abs();
        
        let mut j = distance_size;
        while j > 0 && distance_arr[j - 1] < distance {
            if j < k as usize {
                distance_arr[j] = distance_arr[j - 1];
            }
            j -= 1;
        }
        if j < k as usize {
            distance_arr[j] = distance;
            if distance_size < k as usize {
                distance_size += 1;
            }
        }
        
        if distance_size == k as usize {
            result[i] = distance_arr[k as usize - 1];
        }
    }
    
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line containing queriesSize and k
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let queries_size: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read the queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap()?;
        let mut iter = line.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        queries.push(vec![x, y]);
    }
    
    // Process the queries and get the result
    let result = results_array(&queries, k);
    
    // Print the result
    for i in 0..result.len() {
        print!("{} ", result[i]);
    }
    
    Ok(())
}