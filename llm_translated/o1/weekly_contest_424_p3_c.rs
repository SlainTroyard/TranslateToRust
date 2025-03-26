// A direct translation from the provided C code to Rust, preserving the exact
// algorithm, input format, and output format. Comments are added for clarity.

use std::io::{self, Read};

/// Translates the C function minZeroArray into Rust.
/// Returns the minimum number of queries needed to make all required increments,
/// or -1 if it's impossible (matching the C function's return specification).
fn min_zero_array(nums: &[i32], queries: &[[i32; 3]]) -> i32 {
    // Equivalent checks: if nums is null or numsSize == 0, return 0;
    if nums.is_empty() {
        return 0;
    }
    // If queries is null or queriesSize == 0, return -1;
    if queries.is_empty() {
        return -1;
    }

    // Allocate a difference array (like the C code's "max" array), size = numsSize + 1
    let nums_size = nums.len();
    // In C code: int* max = malloc(sizeof(int) * (numsSize+1));
    // and memset(max, 0, sizeof(int) * numsSize);
    let mut diff = vec![0; nums_size + 1];

    let mut sum = 0;
    let mut k: usize = 0; // Tracks how many queries have been used

    // For each index i in nums:
    for i in 0..nums_size {
        // While we still need more increments to satisfy nums[i]:
        while sum + diff[i] < nums[i] {
            // If we've used up all queries, return -1
            if k == queries.len() {
                return -1;
            }
            let [start, end, increment] = queries[k];
            k += 1;

            // If i is already beyond the 'end' of this query's range, skip
            if (i as i32) > end {
                continue;
            }

            // If current index i is greater than 'start', apply increment at i
            // Otherwise, apply increment at 'start'
            if (i as i32) > start {
                diff[i] += increment;
            } else {
                diff[start as usize] += increment;
            }

            // Decrement after 'end+1' to maintain difference array logic
            let end_plus_one = (end + 1) as usize;
            if end_plus_one < diff.len() {
                diff[end_plus_one] -= increment;
            }
        }
        // Accumulate the difference array increment into sum
        sum += diff[i];
    }

    // Return how many queries were used
    k as i32
}

fn main() {
    // Read entire input from stdin into a single string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Split by whitespace and parse into integers
    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // Use an index to iterate through tokens in order
    let mut idx = 0;

    // 1) Read numsSize (int)
    let nums_size = tokens[idx];
    idx += 1;

    // 2) Read nums array
    let mut nums = Vec::with_capacity(nums_size as usize);
    for _ in 0..nums_size {
        nums.push(tokens[idx]);
        idx += 1;
    }

    // 3) Read queriesSize (int)
    let queries_size = tokens[idx];
    idx += 1;

    // 4) Read each query (start, end, increment)
    let mut queries = Vec::with_capacity(queries_size as usize);
    for _ in 0..queries_size {
        let start = tokens[idx];
        idx += 1;
        let end = tokens[idx];
        idx += 1;
        let inc = tokens[idx];
        idx += 1;
        queries.push([start, end, inc]);
    }

    // Call the function (equivalent to minZeroArray in the C code)
    let result = min_zero_array(&nums, &queries);

    // Print the result just like printf("%d\n", result) in C
    println!("{}", result);
}