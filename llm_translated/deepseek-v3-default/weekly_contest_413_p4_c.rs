use std::cmp::max;
use std::io::{self, BufRead};

fn maximum_subarray_xor(nums: &[i32], queries: &[[usize; 2]]) -> Vec<i32> {
    let n = nums.len();
    let mut subarray_xors = vec![vec![0; n]; n];
    let mut max_scores = vec![vec![0; n]; n];

    // Initialize subarray_xors and max_scores for subarrays of length 1
    for i in 0..n {
        subarray_xors[i][i] = nums[i];
        max_scores[i][i] = nums[i];
    }

    // Compute subarray_xors and max_scores for subarrays of length >= 2
    for sub_length in 2..=n {
        for i in 0..=(n - sub_length) {
            let j = i + sub_length - 1;
            subarray_xors[i][j] = subarray_xors[i][j - 1] ^ subarray_xors[i + 1][j];
            max_scores[i][j] = max(
                subarray_xors[i][j],
                max(max_scores[i][j - 1], max_scores[i + 1][j]),
            );
        }
    }

    // Prepare the answer for each query
    queries.iter().map(|&[l, r]| max_scores[l][r]).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read numsSize
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read nums
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read queriesSize
    let queries_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let query: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push([query[0], query[1]]);
    }

    // Compute the answer
    let answer = maximum_subarray_xor(&nums, &queries);

    // Print the answer
    for &ans in &answer {
        print!("{} ", ans);
    }
    println!();
}