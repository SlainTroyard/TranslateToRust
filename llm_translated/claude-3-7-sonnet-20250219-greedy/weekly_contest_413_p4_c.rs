use std::io::{self, BufRead};
use std::cmp;

fn maximum_subarray_xor(nums: &[i32], queries: &Vec<Vec<i32>>) -> Vec<i32> {
    let nums_size = nums.len();
    let queries_size = queries.len();
    
    // Initialize subarrayXors
    let mut subarray_xors = vec![vec![0; nums_size]; nums_size];
    for i in 0..nums_size {
        subarray_xors[i][i] = nums[i];
    }
    
    // Calculate XOR for all subarrays
    for sub_length in 2..=nums_size {
        let mut i = 0;
        let mut j = sub_length - 1;
        while j < nums_size {
            subarray_xors[i][j] = subarray_xors[i][j - 1] ^ subarray_xors[i + 1][j];
            i += 1;
            j += 1;
        }
    }
    
    // Initialize maxScores
    let mut max_scores = vec![vec![0; nums_size]; nums_size];
    for i in 0..nums_size {
        max_scores[i][i] = nums[i];
    }
    
    // Calculate maximum XOR for all subarrays
    for sub_length in 2..=nums_size {
        let mut i = 0;
        let mut j = sub_length - 1;
        while j < nums_size {
            max_scores[i][j] = cmp::max(
                subarray_xors[i][j],
                cmp::max(max_scores[i][j - 1], max_scores[i + 1][j])
            );
            i += 1;
            j += 1;
        }
    }
    
    // Generate answers for queries
    let mut answer = vec![0; queries_size];
    for i in 0..queries_size {
        let start = queries[i][0] as usize;
        let end = queries[i][1] as usize;
        answer[i] = max_scores[start][end];
    }
    
    answer
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read nums size
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read nums array
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read queries size
    let queries_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let query: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }
    
    // Calculate and print the result
    let answer = maximum_subarray_xor(&nums, &queries);
    
    // Print the answer
    for (i, &val) in answer.iter().enumerate() {
        print!("{}", val);
        if i < answer.len() - 1 {
            print!(" ");
        }
    }
    println!();
}