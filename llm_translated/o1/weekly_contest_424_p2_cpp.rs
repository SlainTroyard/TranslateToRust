use std::error::Error;
use std::io::{self, Read};

// A struct to mirror the C++ class "Solution"
struct Solution;

impl Solution {
    // This function replicates the C++ "isZeroArray" method exactly
    fn is_zero_array(&self, nums: &[i32], queries: &[[i32; 2]]) -> bool {
        let n = nums.len();
        
        // Create a difference array with one extra element for the endpoint+1 operation
        let mut v = vec![0; n + 1];

        // For each query [start, end], increment at start index and decrement just after end
        for &query in queries {
            let start = query[0] as usize;
            let end = query[1] as usize;
            v[start] += 1;
            v[end + 1] -= 1; // Assuming input guarantees end+1 < v.len()
        }

        // Build the prefix sum to apply all increments/decrements
        for i in 1..n {
            v[i] += v[i - 1];
        }

        // Check if any element in nums remains > 0 after applying the query operations
        for i in 0..n {
            if nums[i] - v[i] > 0 {
                return false;
            }
        }
        true
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read entire input from stdin as a single string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input on whitespace and parse each piece into an i32
    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    // We use an index to traverse the parsed tokens
    let mut idx = 0;

    // Read the size of the nums array
    let n = tokens[idx] as usize;
    idx += 1;

    // Read the nums array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        nums.push(tokens[idx]);
        idx += 1;
    }

    // Read the number of queries
    let m = tokens[idx] as usize;
    idx += 1;

    // Read each query as [start, end]
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        let start = tokens[idx];
        let end = tokens[idx + 1];
        idx += 2;
        queries.push([start, end]);
    }

    // Create a Solution instance and call is_zero_array
    let sol = Solution;
    if sol.is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}