use std::io::{self, BufRead, Write};

/// Function that mimics the behavior of the given C function.
/// It calculates the minimum number of queries needed such that for every position i,
/// the running value (sum) plus adjustments (from max array) is at least nums[i].
///
/// Returns:
///   - The number of queries used if successful.
///   - -1 if it is not possible or if queries are exhausted.
fn min_zero_array(nums: &[i32], queries: &[[i32; 3]]) -> i32 {
    let nums_size = nums.len();
    if nums.is_empty() {
        return 0;
    }
    if queries.is_empty() {
        return -1;
    }

    // Allocate a vector 'max' with size nums_size + 1, initialized with 0.
    // Note: In the C code, they allocated nums_size+1 elements.
    let mut max = vec![0; nums_size + 1];

    let mut sum = 0;
    let mut k = 0;
    let queries_size = queries.len();

    // Iterate through each index in nums.
    for i in 0..nums_size {
        // Update the max array until the current sum satisfies nums[i]
        while sum + max[i] < nums[i] {
            if k == queries_size {
                // ran out of queries
                return -1;
            }
            // Get the current query: [start, end, increment]
            let start = queries[k][0] as usize;
            let end = queries[k][1] as usize;
            let increment = queries[k][2];
            k += 1;

            // If i is greater than end, skip this query (it doesn't affect index i)
            if i > end {
                continue;
            }

            // Apply the increment in a difference array style.
            // If i > start, add at index i; otherwise, add at index start.
            if i > start {
                max[i] += increment;
            } else {
                // ensure start is within bounds (it should be as per input)
                if start < max.len() {
                    max[start] += increment;
                }
            }
            // Subtract the increment past the 'end' of the query range.
            if end + 1 < max.len() {
                max[end + 1] -= increment;
            }
        }
        // Update our cumulative sum
        sum += max[i];
    }
    k as i32
}

fn main() -> io::Result<()> {
    // Use a locked stdin reader for fast I/O.
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // Buffer for reading lines.
    let mut buffer = String::new();

    // Read numsSize
    buffer.clear();
    reader.read_line(&mut buffer)?;
    let nums_size: usize = buffer.trim().parse().expect("Failed to parse numsSize");

    // Read nums array
    let mut nums = Vec::with_capacity(nums_size);
    // We need to read nums_size integers.
    // The input might be in one or multiple lines: use loop until we've parsed enough numbers.
    while nums.len() < nums_size {
        buffer.clear();
        reader.read_line(&mut buffer)?;
        // Split by whitespace and parse integers.
        for token in buffer.split_whitespace() {
            if nums.len() >= nums_size {
                break;
            }
            nums.push(token.parse::<i32>().expect("Failed to parse a number in nums"));
        }
    }

    // Read queriesSize
    buffer.clear();
    reader.read_line(&mut buffer)?;
    let queries_size: usize = buffer.trim().parse().expect("Failed to parse queriesSize");

    // Read queries. Each query has 3 integers.
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        // We need to read until 3 integers are fetched for this query.
        let mut query_vec = Vec::new();
        while query_vec.len() < 3 {
            buffer.clear();
            reader.read_line(&mut buffer)?;
            for token in buffer.split_whitespace() {
                if query_vec.len() >= 3 {
                    break;
                }
                query_vec.push(token.parse::<i32>().expect("Failed to parse a query element"));
            }
        }
        // Convert the query_vec to an array of 3 elements.
        let query_array: [i32; 3] = [query_vec[0], query_vec[1], query_vec[2]];
        queries.push(query_array);
    }

    // Call the min_zero_array function.
    let result = min_zero_array(&nums, &queries);

    // Write the result to stdout, following the exact same format as C code.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", result)?;

    Ok(())
}