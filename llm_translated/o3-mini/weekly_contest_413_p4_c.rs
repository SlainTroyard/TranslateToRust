use std::cmp;
use std::io::{self, BufRead, Write};

fn maximum_subarray_xor(nums: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    let n = nums.len();

    // Allocate 2D vectors for subarray_xors and max_scores
    // subarray_xors[i][j] holds the XOR for the subarray nums[i..=j]
    let mut subarray_xors = vec![vec![0; n]; n];
    // Initialize the diagonal: subarray_xors[i][i] = nums[i]
    for i in 0..n {
        subarray_xors[i][i] = nums[i];
    }
    // Compute subarray XOR for all subarrays of length >= 2
    for sub_len in 2..=n {
        // i: starting index, j = i + sub_len - 1 as the ending index
        for i in 0..=n - sub_len {
            let j = i + sub_len - 1;
            subarray_xors[i][j] = subarray_xors[i][j - 1] ^ subarray_xors[i + 1][j];
        }
    }

    // Allocate and compute max_scores matrix
    // max_scores[i][j] holds the maximum value among:
    // subarray_xors[i][j], max_scores[i][j-1], max_scores[i+1][j]
    let mut max_scores = vec![vec![0; n]; n];
    for i in 0..n {
        max_scores[i][i] = nums[i];
    }
    // Compute maximum scores for every subarray
    for sub_len in 2..=n {
        for i in 0..=n - sub_len {
            let j = i + sub_len - 1;
            // Compute the maximum among subarray_xors[i][j], max_scores[i][j-1], and max_scores[i+1][j]
            max_scores[i][j] = cmp::max(
                subarray_xors[i][j],
                cmp::max(max_scores[i][j - 1], max_scores[i + 1][j]),
            );
        }
    }

    // Process queries and build the answers vector.
    // Each query gives two indices; fetch the precomputed maximum from max_scores.
    let mut answer = Vec::with_capacity(queries.len());
    for &(i, j) in queries {
        answer.push(max_scores[i][j]);
    }
    answer
}

fn main() -> io::Result<()> {
    // Using stdin lock for faster I/O
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input_line = String::new();

    // Read number of elements in nums array
    input_line.clear();
    reader.read_line(&mut input_line)?;
    let nums_size: usize = input_line
        .trim()
        .parse()
        .expect("Failed to parse number of elements");

    // Read the nums array elements
    input_line.clear();
    // If the numbers might be on one or more lines, read as many tokens as required.
    // Accumulate tokens until we have enough numbers.
    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    while nums.len() < nums_size {
        input_line.clear();
        reader.read_line(&mut input_line)?;
        for token in input_line.split_whitespace() {
            if nums.len() < nums_size {
                let num: i32 = token.parse().expect("Failed to parse an integer in nums");
                nums.push(num);
            } else {
                break;
            }
        }
    }

    // Read number of queries
    input_line.clear();
    reader.read_line(&mut input_line)?;
    let queries_size: usize = input_line
        .trim()
        .parse()
        .expect("Failed to parse number of queries");

    // Read each query, each query is two integers.
    let mut queries = Vec::with_capacity(queries_size);
    let mut tokens = Vec::<String>::new();
    let mut total_query_tokens = 0;
    // Since queries are read one by one (the original code reads a pair per query),
    // we can loop queries_size times to get two integers per query.
    while queries.len() < queries_size {
        input_line.clear();
        reader.read_line(&mut input_line)?;
        // Split the line into whitespace separated tokens
        for token in input_line.split_whitespace() {
            tokens.push(token.to_string());
        }
        // Check if we have at least 2 tokens for one query
        while tokens.len() >= 2 && queries.len() < queries_size {
            let first = tokens.remove(0);
            let second = tokens.remove(0);
            let start: usize = first.parse().expect("Failed to parse query index");
            let end: usize = second.parse().expect("Failed to parse query index");
            queries.push((start, end));
        }
    }

    // Compute answer using precomputed max_scores from maximum_subarray_xor function.
    let answer = maximum_subarray_xor(&nums, &queries);

    // Write the answer to stdout exactly as required.
    // Each result is printed followed by a space, all on one line, ending with a newline.
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    for num in answer {
        write!(writer, "{} ", num)?;
    }
    writeln!(writer)?;

    Ok(())
}