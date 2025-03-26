use std::io::{self, BufRead};
use std::cmp;

fn maximum_subarray_xor(nums: &[i32], queries: &[Vec<i32>]) -> Vec<i32> {
    let nums_size = nums.len();
    let queries_size = queries.len();
    
    // Initialize subarrayXors
    let mut subarray_xors = vec![vec![0; nums_size]; nums_size];
    for i in 0..nums_size {
        subarray_xors[i][i] = nums[i];
    }
    
    // Compute XOR for all subarrays
    for sub_length in 2..=nums_size {
        for i in 0..=(nums_size - sub_length) {
            let j = i + sub_length - 1;
            subarray_xors[i][j] = subarray_xors[i][j-1] ^ nums[j]; // A more efficient way to compute
        }
    }
    
    // Initialize maxScores
    let mut max_scores = vec![vec![0; nums_size]; nums_size];
    for i in 0..nums_size {
        max_scores[i][i] = nums[i];
    }
    
    // Compute maximum XOR for all subarray ranges
    for sub_length in 2..=nums_size {
        for i in 0..=(nums_size - sub_length) {
            let j = i + sub_length - 1;
            max_scores[i][j] = cmp::max(
                subarray_xors[i][j],
                cmp::max(max_scores[i][j-1], max_scores[i+1][j])
            );
        }
    }
    
    // Compute answers for queries
    let mut answer = vec![0; queries_size];
    for i in 0..queries_size {
        let start = queries[i][0] as usize;
        let end = queries[i][1] as usize;
        answer[i] = max_scores[start][end];
    }
    
    answer
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read nums size
    let nums_size: usize = lines.next().unwrap()?.trim().parse().expect("Failed to parse nums_size");
    
    // Read nums array
    let nums_line = lines.next().unwrap()?;
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse num"))
        .collect();
    
    // Read queries size
    let queries_size: usize = lines.next().unwrap()?.trim().parse().expect("Failed to parse queries_size");
    
    // Read queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let query_line = lines.next().unwrap()?;
        let query: Vec<i32> = query_line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Failed to parse query"))
            .collect();
        queries.push(query);
    }
    
    // Calculate and print the results
    let answer = maximum_subarray_xor(&nums, &queries);
    for result in answer {
        print!("{} ", result);
    }
    println!();
    
    Ok(())
}