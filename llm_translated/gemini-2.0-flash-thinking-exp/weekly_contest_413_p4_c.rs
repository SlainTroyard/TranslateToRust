use std::io::{self, BufRead, Write};
use std::cmp::max;
use std::vec;

fn maximum_subarray_xor(nums: &[i32], queries: &Vec<Vec<i32>>) -> Vec<i32> {
    let nums_size = nums.len();
    let queries_size = queries.len();

    // Initialize subarrayXors and maxScores as 2D vectors
    let mut subarray_xors = vec![vec![0; nums_size]; nums_size];
    let mut max_scores = vec![vec![0; nums_size]; nums_size];

    // Calculate subarrayXors
    for i in 0..nums_size {
        subarray_xors[i][i] = nums[i];
    }
    for sub_length in 2..=nums_size {
        for i in 0..=(nums_size - sub_length) {
            let j = i + sub_length - 1;
            subarray_xors[i][j] = subarray_xors[i][j - 1] ^ subarray_xors[i + 1][j];
        }
    }

    // Calculate maxScores
    for i in 0..nums_size {
        max_scores[i][i] = nums[i];
    }
    for sub_length in 2..=nums_size {
        for i in 0..=(nums_size - sub_length) {
            let j = i + sub_length - 1;
            max_scores[i][j] = max(subarray_xors[i][j], max(max_scores[i][j - 1], max_scores[i + 1][j]));
        }
    }

    // Prepare answer vector
    let mut answer = Vec::with_capacity(queries_size);
    for i in 0..queries_size {
        answer.push(max_scores[queries[i][0] as usize][queries[i][1] as usize]);
    }

    answer
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout);

    // Read numsSize
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let nums_size: usize = line.trim().parse().unwrap();

    // Read nums
    line.clear();
    reader.read_line(&mut line)?;
    let nums: Vec<i32> = line.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read queriesSize
    line.clear();
    reader.read_line(&mut line)?;
    let queries_size: usize = line.trim().parse().unwrap();

    // Read queries
    let mut queries: Vec<Vec<i32>> = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        line.clear();
        reader.read_line(&mut line)?;
        let query: Vec<i32> = line.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }

    // Call maximumSubarrayXor
    let answer = maximum_subarray_xor(&nums, &queries);

    // Print answer
    for i in 0..answer.len() {
        write!(writer, "{} ", answer[i])?;
    }
    writeln!(writer)?;

    writer.flush()?;
    Ok(())
}