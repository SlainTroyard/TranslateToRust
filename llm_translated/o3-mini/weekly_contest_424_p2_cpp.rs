use std::error::Error;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    // Acquire a locked handle to standard input.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first token: n, the size of the nums array.
    let n = read_next_token(&mut lines)?.parse::<usize>()?;

    // Read n integers for the nums array. They may be spread across multiple lines.
    let mut nums = Vec::with_capacity(n);
    while nums.len() < n {
        let line = read_next_line(&mut lines)?;
        for token in line.split_whitespace() {
            if nums.len() >= n {
                break;
            }
            nums.push(token.parse::<i32>()?);
        }
    }

    // Read the next token: m, the number of queries.
    let m = read_next_token(&mut lines)?.parse::<usize>()?;

    // Read m queries. Each query consists of exactly two integers.
    let mut queries = Vec::with_capacity(m);
    while queries.len() < m {
        // We might get both values on one line or need to process tokens across lines.
        let line = read_next_line(&mut lines)?;
        let mut tokens = line.split_whitespace();
        // To ensure we get two numbers, if not sufficient tokens on that line, keep reading.
        let mut query = Vec::with_capacity(2);
        while query.len() < 2 {
            if let Some(token) = tokens.next() {
                query.push(token.parse::<usize>()?);
            } else {
                // Not enough tokens in this line, try next line.
                let new_line = read_next_line(&mut lines)?;
                tokens = new_line.split_whitespace();
            }
        }
        queries.push(query);
    }

    // Call the solution function.
    let result = is_zero_array(&nums, &queries);

    // Print the result exactly as "true" or "false" followed by a newline.
    println!("{}", if result { "true" } else { "false" });
    Ok(())
}

// Helper function to read the next non-empty line.
fn read_next_line(lines: &mut impl Iterator<Item = Result<String, io::Error>>) -> Result<String, Box<dyn Error>> {
    for line in lines {
        let s = line?;
        if !s.trim().is_empty() {
            return Ok(s);
        }
    }
    Err("Unexpected end of input".into())
}

// Helper function to read the next token from the input.
fn read_next_token(lines: &mut impl Iterator<Item = Result<String, io::Error>>) -> Result<String, Box<dyn Error>> {
    // We loop until we find a token.
    loop {
        let line = read_next_line(lines)?;
        let mut tokens = line.split_whitespace();
        if let Some(token) = tokens.next() {
            return Ok(token.to_string());
        }
    }
}

// This function implements the core logic of the solution.
// It follows the same algorithm as the provided C++ code.
fn is_zero_array(nums: &Vec<i32>, queries: &Vec<Vec<usize>>) -> bool {
    // Create a difference array v with length equal to nums.len() + 1, initialized to 0.
    let mut v = vec![0; nums.len() + 1];

    // For each query, update the difference array.
    for query in queries {
        let start = query[0];
        let end = query[1];
        v[start] += 1;
        // Check bounds before subtracting.
        if end + 1 < v.len() {
            v[end + 1] -= 1;
        }
    }

    // Compute the prefix sum to rebuild the applied increments.
    for i in 1..nums.len() {
        v[i] += v[i - 1];
    }

    // Check if for every index, nums[i] - v[i] is not positive.
    for i in 0..nums.len() {
        if nums[i] - v[i] > 0 {
            return false;
        }
    }

    true
}