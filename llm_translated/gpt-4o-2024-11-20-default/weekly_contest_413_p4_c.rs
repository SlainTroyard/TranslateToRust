use std::io::{self, BufRead};
use std::cmp;

fn maximum_subarray_xor(nums: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    let nums_size = nums.len();
    
    // Precompute subarray XORs
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

    // Precompute maximum scores
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

    // Calculate answers for queries
    queries
        .iter()
        .map(|&(l, r)| max_scores[l][r])
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse nums
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(nums.len(), nums_size);

    // Parse queries
    let queries_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let query: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        queries.push((query[0], query[1]));
    }

    // Compute results
    let results = maximum_subarray_xor(&nums, &queries);

    // Print results
    for res in results {
        print!("{} ", res);
    }
    println!();
}