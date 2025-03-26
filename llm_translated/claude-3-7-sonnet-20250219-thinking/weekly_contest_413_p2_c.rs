use std::io::{self, BufRead};

fn results_array(queries: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let queries_size = queries.len();
    let mut result = vec![-1; queries_size];
    
    // Array to store the k largest distances
    let mut distance_arr = vec![0; (k + 1) as usize];
    let mut distance_size = 0;
    
    for i in 0..queries_size {
        // Calculate Manhattan distance
        let distance = queries[i][0].abs() + queries[i][1].abs();
        
        // Find the correct insertion position
        let mut j = distance_size;
        while j > 0 && distance_arr[j - 1] < distance {
            if j < k as usize {
                distance_arr[j] = distance_arr[j - 1];
            }
            j -= 1;
        }
        
        // Insert the new distance if it's among the k largest
        if j < k as usize {
            distance_arr[j] = distance;
            if distance_size < k as usize {
                distance_size += 1;
            }
        }
        
        // Set the result if we have k distances
        if distance_size == k as usize {
            result[i] = distance_arr[k as usize - 1];
        }
    }
    
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read queries_size and k
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<i32> = first_line.split_whitespace()
        .map(|s| s.parse().expect("Failed to parse input"))
        .collect();
    
    let queries_size = parts[0] as usize;
    let k = parts[1];
    
    // Read queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Failed to parse coordinate"))
            .collect();
        
        queries.push(vec![coords[0], coords[1]]);
    }
    
    // Process queries and get results
    let result = results_array(&queries, k);
    
    // Print results with exact same format as the C code
    for val in result {
        print!("{} ", val);
    }
}