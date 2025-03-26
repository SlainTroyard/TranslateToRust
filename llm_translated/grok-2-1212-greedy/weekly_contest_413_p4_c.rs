use std::io::{self, BufRead};

fn maximum_subarray_xor(nums: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    let n = nums.len();
    let mut subarray_xors = vec![vec![0; n]; n];
    let mut max_scores = vec![vec![0; n]; n];

    // Initialize subarray_xors and max_scores
    for i in 0..n {
        subarray_xors[i][i] = nums[i];
        max_scores[i][i] = nums[i];
    }

    // Calculate subarray XORs
    for sub_length in 2..=n {
        for i in 0..=n - sub_length {
            let j = i + sub_length - 1;
            subarray_xors[i][j] = subarray_xors[i][j - 1] ^ subarray_xors[i + 1][j];
        }
    }

    // Calculate maximum scores
    for sub_length in 2..=n {
        for i in 0..=n - sub_length {
            let j = i + sub_length - 1;
            max_scores[i][j] = subarray_xors[i][j].max(max_scores[i][j - 1].max(max_scores[i + 1][j]));
        }
    }

    // Process queries
    queries.iter().map(|&(from, to)| max_scores[from][to]).collect()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read nums
    let nums_size: usize = lines.next().unwrap()?.parse().unwrap();
    let nums: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .take(nums_size)
        .collect();

    // Read queries
    let queries_size: usize = lines.next().unwrap()?.parse().unwrap();
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let query: Vec<usize> = lines.next().unwrap()?.split_whitespace()
            .take(2)
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push((query[0], query[1]));
    }

    // Calculate and print results
    let result = maximum_subarray_xor(&nums, &queries);
    for num in result {
        print!("{} ", num);
    }
    println!();

    Ok(())
}