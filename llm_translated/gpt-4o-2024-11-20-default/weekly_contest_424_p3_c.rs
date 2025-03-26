// Problem: Weekly Contest 424 Problem 3
use std::io;
use std::cmp::max;
use std::vec::Vec;

fn min_zero_array(nums: Vec<i32>, queries: Vec<(usize, usize, i32)>) -> i32 {
    let nums_size = nums.len();

    if nums.is_empty() || nums_size == 0 {
        return 0;
    }

    if queries.is_empty() {
        return -1;
    }

    // Equivalent to `max` array in C
    let mut max_buffer = vec![0; nums_size + 1]; // One extra index for 'end + 1'
    let mut sum = 0;
    let mut query_index = 0;

    for i in 0..nums_size {
        while sum + max_buffer[i] < nums[i] {
            // If we've exhausted all queries, return -1
            if query_index == queries.len() {
                return -1;
            }

            let (start, end, increment) = queries[query_index];
            query_index += 1;

            if i > end {
                continue;
            }

            if i >= start {
                max_buffer[i] += increment;
            } else {
                max_buffer[start] += increment;
            }
            max_buffer[end + 1] -= increment;
        }

        sum += max_buffer[i];
    }

    query_index as i32
}

fn main() {
    let mut input = String::new();
    let mut stdin = io::stdin();

    // Reading nums array size
    stdin.read_line(&mut input).unwrap();
    let nums_size: usize = input.trim().parse().unwrap();
    input.clear();

    // Reading nums array
    stdin.read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    input.clear();

    // Reading queries size
    stdin.read_line(&mut input).unwrap();
    let query_size: usize = input.trim().parse().unwrap();
    input.clear();

    // Reading query data
    let mut queries = Vec::with_capacity(query_size);
    for _ in 0..query_size {
        stdin.read_line(&mut input).unwrap();
        let query: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        input.clear();
        queries.push((query[0] as usize, query[1] as usize, query[2]));
    }

    // Calling the function
    let result = min_zero_array(nums, queries);

    // Output the result
    println!("{}", result);
}