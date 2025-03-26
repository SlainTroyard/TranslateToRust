use std::io::{self, Read};

fn maximum_subarray_xor(nums: &[i32], queries: &[[usize; 2]]) -> Vec<i32> {
    let nums_size = nums.len();
    if nums_size == 0 {
        return Vec::new();
    }

    // Initialize subarray_xors matrix with diagonals set to nums[i]
    let mut subarray_xors = vec![vec![0; nums_size]; nums_size];
    for i in 0..nums_size {
        subarray_xors[i][i] = nums[i];
    }

    // Fill subarray_xors for subarrays of length >=2 using the recurrence relation
    for sub_length in 2..=nums_size {
        for i in 0..=(nums_size - sub_length) {
            let j = i + sub_length - 1;
            subarray_xors[i][j] = subarray_xors[i][j - 1] ^ subarray_xors[i + 1][j];
        }
    }

    // Initialize max_scores matrix with diagonals set to nums[i]
    let mut max_scores = vec![vec![0; nums_size]; nums_size];
    for i in 0..nums_size {
        max_scores[i][i] = nums[i];
    }

    // Fill max_scores for subarrays of length >=2 by taking maximum of current XOR and neighbors
    for sub_length in 2..=nums_size {
        for i in 0..=(nums_size - sub_length) {
            let j = i + sub_length - 1;
            let current_xor = subarray_xors[i][j];
            let left_max = max_scores[i][j - 1];
            let right_max = max_scores[i + 1][j];
            max_scores[i][j] = current_xor.max(left_max).max(right_max);
        }
    }

    // Process each query by looking up precomputed max_scores
    queries.iter()
        .map(|&query| max_scores[query[0]][query[1]])
        .collect()
}

fn main() {
    // Read entire input as whitespace-separated tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Parse nums array
    let nums_size = tokens.next().unwrap().parse::<usize>().unwrap();
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        nums.push(tokens.next().unwrap().parse::<i32>().unwrap());
    }

    // Parse queries array
    let queries_size = tokens.next().unwrap().parse::<usize>().unwrap();
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let l = tokens.next().unwrap().parse::<usize>().unwrap();
        let r = tokens.next().unwrap().parse::<usize>().unwrap();
        queries.push([l, r]);
    }

    // Compute answers using the optimized dynamic programming approach
    let answers = maximum_subarray_xor(&nums, &queries);

    // Print answers in space-separated format
    for ans in answers {
        print!("{} ", ans);
    }
    println!();
}