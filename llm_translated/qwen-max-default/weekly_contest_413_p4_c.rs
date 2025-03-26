use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the size of nums
    stdin.read_line(&mut buffer).unwrap();
    let nums_size: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read nums
    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        stdin.read_line(&mut buffer).unwrap();
        nums.push(buffer.trim().parse().unwrap());
        buffer.clear();
    }

    // Read the size of queries
    stdin.read_line(&mut buffer).unwrap();
    let queries_size: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read queries
    let mut queries: Vec<(usize, usize)> = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        stdin.read_line(&mut buffer).unwrap();
        let parts: Vec<&str> = buffer.trim().split_whitespace().collect();
        let start: usize = parts[0].parse().unwrap();
        let end: usize = parts[1].parse().unwrap();
        queries.push((start, end));
        buffer.clear();
    }

    // Calculate the maximum subarray XOR
    let answer = maximum_subarray_xor(&nums, &queries);

    // Print the results
    for &result in &answer {
        write!(stdout, "{} ", result).unwrap();
    }
    writeln!(stdout).unwrap();
}

/// Translates the C function `maximumSubarrayXor` to Rust
fn maximum_subarray_xor(nums: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    let nums_size = nums.len();
    let queries_size = queries.len();

    // Initialize subarrayXors and maxScores
    let mut subarray_xors: Vec<Vec<i32>> = vec![vec![0; nums_size]; nums_size];
    let mut max_scores: Vec<Vec<i32>> = vec![vec![0; nums_size]; nums_size];

    // Fill the diagonal with the values from nums
    for i in 0..nums_size {
        subarray_xors[i][i] = nums[i];
        max_scores[i][i] = nums[i];
    }

    // Fill subarrayXors
    for sub_length in 2..=nums_size {
        for (i, j) in (0..nums_size - sub_length + 1).zip(sub_length - 1..nums_size) {
            subarray_xors[i][j] = subarray_xors[i][j - 1] ^ subarray_xors[i + 1][j];
        }
    }

    // Fill maxScores
    for sub_length in 2..=nums_size {
        for (i, j) in (0..nums_size - sub_length + 1).zip(sub_length - 1..nums_size) {
            max_scores[i][j] = subarray_xors[i][j].max(max_scores[i][j - 1].max(max_scores[i + 1][j]));
        }
    }

    // Collect the answers for each query
    let mut answer: Vec<i32> = Vec::with_capacity(queries_size);
    for &(start, end) in queries {
        answer.push(max_scores[start][end]);
    }

    answer
}