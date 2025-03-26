/// Translation of the C program for LeetCode Weekly Contest 424 Problem 2
/// to idiomatic Rust. This program reads input from stdin and writes output
/// to stdout, preserving the exact I/O format of the original C code.
///
/// The input format is:
/// 1) One integer (numsSize): size of the nums array
/// 2) numsSize integers for nums
/// 3) One integer (queriesSize): number of queries
/// 4) queriesSize lines, each with two integers (start, end) for each query
///
/// The output is a single line, "true" or "false", followed by a newline.
use std::io::{self, BufRead};

/// Checks if the array `nums` can be "zeroed out" by applying the increment
/// queries. Each query increments the range [start, end] in an implicit
/// difference array. If at any point `nums[i]` is greater than the available
/// incremented count, the function returns false. Otherwise, it returns true.
fn is_zero_array(nums: &[i32], queries: &[[i32; 2]]) -> bool {
    let nums_size = nums.len();
    let mut diff = vec![0; nums_size];
    let mut count = 0;

    // Build the difference array
    for query in queries {
        let start = query[0] as usize;
        let end = query[1] as usize;
        diff[start] += 1;
        if end + 1 < nums_size {
            diff[end + 1] -= 1;
        }
    }

    // Accumulate the differences and compare with nums
    for i in 0..nums_size {
        count += diff[i];
        if nums[i] > count {
            return false;
        }
    }

    true
}

fn main() -> io::Result<()> {
    // Read all tokens from stdin (integers only), ignoring line boundaries
    let stdin = io::stdin();
    let tokens: Vec<i32> = stdin
        .lock()
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|word| word.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    // Parse tokens in the same order as the original C program
    let mut index = 0;

    // Read numsSize
    let nums_size = tokens[index] as usize;
    index += 1;

    // Read nums array
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        nums.push(tokens[index]);
        index += 1;
    }

    // Read queriesSize
    let queries_size = tokens[index] as usize;
    index += 1;

    // Read queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let start = tokens[index];
        let end = tokens[index + 1];
        index += 2;
        queries.push([start, end]);
    }

    // Call the function and display the result
    if is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}