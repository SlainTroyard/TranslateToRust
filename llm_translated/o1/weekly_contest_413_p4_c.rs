use std::io::{self, BufRead, Write};
use std::error::Error;
use std::cmp;

/// This function replicates the exact logic of the C function `maximumSubarrayXor`.
/// It computes a 2D table of XOR values for all subarrays of `nums`, then a 2D table
/// of max scores for those subarrays, and finally returns the requested ranges.
fn maximum_subarray_xor(nums: &[i32], queries: &[[usize; 2]]) -> Vec<i32> {
    let n = nums.len();

    // Create a 2D vector storing XOR of subarray from i..j
    // (equivalent to `int** subarrayXors` in the C code)
    let mut subarray_xors = vec![vec![0; n]; n];
    for i in 0..n {
        subarray_xors[i][i] = nums[i];
    }
    // Replicating the redundant loop in the C code
    for i in 0..n {
        subarray_xors[i][i] = nums[i];
    }

    // Fill subarray_xors for all subarray lengths from 2 to n
    for sub_length in 2..=n {
        for i in 0..=n - sub_length {
            let j = i + sub_length - 1;
            subarray_xors[i][j] = subarray_xors[i][j - 1] ^ subarray_xors[i + 1][j];
        }
    }

    // Create a 2D vector storing max XOR values for subarray (equivalent to `int** maxScores`)
    let mut max_scores = vec![vec![0; n]; n];
    for i in 0..n {
        max_scores[i][i] = nums[i];
    }

    // Compute max XOR values using previously computed subarray_xors
    for sub_length in 2..=n {
        for i in 0..=n - sub_length {
            let j = i + sub_length - 1;
            max_scores[i][j] = cmp::max(
                subarray_xors[i][j],
                cmp::max(max_scores[i][j - 1], max_scores[i + 1][j]),
            );
        }
    }

    // Prepare the results for each query
    let mut answer = Vec::with_capacity(queries.len());
    for &query in queries {
        let (left, right) = (query[0], query[1]);
        answer.push(max_scores[left][right]);
    }

    answer
}

/// Main function replicating the exact I/O format from the C code.
/// 1. Reads an integer N (numsSize).
/// 2. Reads N integers into nums.
/// 3. Reads an integer Q (queriesSize).
/// 4. Reads Q lines, each with two integers.
/// 5. Computes the maximumSubarrayXor for the queries.
/// 6. Prints the results space-separated, ending with a newline.
fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut tokens = Vec::new();

    // Read all tokens from stdin, splitting on whitespace (mimics multiple scanf calls).
    for line in stdin.lock().lines() {
        let line = line?;
        for token in line.split_whitespace() {
            tokens.push(token.to_string());
        }
    }

    // We will parse the tokens in the same order as the C code's scanf calls
    let mut index = 0;

    // 1. Read numsSize
    let nums_size: usize = tokens[index].parse()?;
    index += 1;

    // 2. Read the nums array
    let mut nums = vec![0i32; nums_size];
    for i in 0..nums_size {
        nums[i] = tokens[index].parse()?;
        index += 1;
    }

    // 3. Read queriesSize
    let queries_size: usize = tokens[index].parse()?;
    index += 1;

    // 4. Read queries (each query has two integers)
    let mut queries = vec![[0usize; 2]; queries_size];
    for i in 0..queries_size {
        for j in 0..2 {
            queries[i][j] = tokens[index].parse()?;
            index += 1;
        }
    }

    // 5. Compute answers
    let answers = maximum_subarray_xor(&nums, &queries);

    // 6. Print results space-separated, then newline
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for (i, val) in answers.iter().enumerate() {
        write!(handle, "{} ", val)?;
    }
    writeln!(handle)?;

    Ok(())
}