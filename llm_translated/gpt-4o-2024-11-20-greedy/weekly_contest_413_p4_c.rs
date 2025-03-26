use std::io::{self, BufRead};
use std::cmp;
use std::iter;

// Function to compute the maximum subarray xor for the given nums and queries
fn maximum_subarray_xor(nums: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    let nums_size = nums.len();
    
    // Step 1: Create a 2D `subarray_xors` vector to store XOR values of all subarrays
    let mut subarray_xors = vec![vec![0; nums_size]; nums_size];
    
    for i in 0..nums_size {
        subarray_xors[i][i] = nums[i];
    }
    
    for sub_length in 2..=nums_size {
        for i in 0..=(nums_size - sub_length) {
            let j = i + sub_length - 1;
            subarray_xors[i][j] = subarray_xors[i][j - 1] ^ subarray_xors[i + 1][j];
        }
    }
    
    // Step 2: Compute the maximum XOR values for all subarrays
    let mut max_scores = vec![vec![0; nums_size]; nums_size];
    
    for i in 0..nums_size {
        max_scores[i][i] = nums[i];
    }
    
    for sub_length in 2..=nums_size {
        for i in 0..=(nums_size - sub_length) {
            let j = i + sub_length - 1;
            max_scores[i][j] = cmp::max(
                subarray_xors[i][j],
                cmp::max(max_scores[i][j - 1], max_scores[i + 1][j]),
            );
        }
    }
    
    // Step 3: Process the queries
    let mut answer = Vec::with_capacity(queries.len());
    for &(l, r) in queries {
        answer.push(max_scores[l][r]);
    }
    
    answer
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    
    // Step 1: Read `nums` size and elements
    let nums_size: usize = input.next().unwrap().unwrap().trim().parse().unwrap();
    let nums: Vec<i32> = input
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(nums.len(), nums_size);
    
    // Step 2: Read `queries` size and elements
    let queries_size: usize = input.next().unwrap().unwrap().trim().parse().unwrap();
    let mut queries = Vec::with_capacity(queries_size);
    
    for _ in 0..queries_size {
        let query: Vec<usize> = input
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        assert_eq!(query.len(), 2);
        queries.push((query[0], query[1]));
    }
    
    // Step 3: Compute the answer for the queries
    let answer = maximum_subarray_xor(&nums, &queries);
    
    // Step 4: Print the result
    for val in answer {
        print!("{} ", val);
    }
    println!();
}