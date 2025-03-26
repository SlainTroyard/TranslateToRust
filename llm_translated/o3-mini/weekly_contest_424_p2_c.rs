use std::error::Error;
use std::io::{self, Read};

/// Determines if the array `nums` can be “zeroed” by applying the queries.
/// Each query represented by (l, r) indicates that each index in [l, r] has been decreased once.
/// We use a difference array approach to efficiently simulate the decreases.
///
/// # Arguments
/// * `nums` - Slice representing the numbers array.
/// * `queries` - Slice of tuple queries where each tuple is (l, r).
///
/// # Returns
/// * `true` if for every index the applied decreases (prefix sum) are at least as much as `nums[index]`.
/// * `false` otherwise.
fn is_zero_array(nums: &[i32], queries: &[(usize, usize)]) -> bool {
    let n = nums.len();
    // Create a diff vector of length `n`, initialized with zeros.
    let mut diff = vec![0; n];
    
    // Process each query, updating the difference array.
    for &(l, r) in queries {
        diff[l] += 1; // Start the increment at index l.
        if r + 1 < n {
            diff[r + 1] -= 1; // End the increment after index r.
        }
    }
    
    // Simulate prefix sum on the diff array and check with the corresponding nums element.
    let mut count = 0;
    for i in 0..n {
        count += diff[i];
        if nums[i] > count {
            return false;
        }
    }
    
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input from standard input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Split the input by whitespace to get individual tokens
    let mut tokens = input.split_whitespace();
    
    // Read the size of the nums array.
    let nums_size: usize = tokens
        .next()
        .ok_or("Expected nums size")?
        .parse()
        .map_err(|_| "Invalid nums size")?;
    
    // Read the nums array elements.
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let value = tokens
            .next()
            .ok_or("Expected nums element")?
            .parse::<i32>()
            .map_err(|_| "Invalid nums element")?;
        nums.push(value);
    }
    
    // Read the number of queries.
    let queries_size: usize = tokens
        .next()
        .ok_or("Expected queries size")?
        .parse()
        .map_err(|_| "Invalid queries size")?;
    
    // Read each query (each query contains exactly two integers).
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        // Read the starting and ending indices for the query.
        let l = tokens
            .next()
            .ok_or("Expected query start index")?
            .parse::<usize>()
            .map_err(|_| "Invalid query start index")?;
        let r = tokens
            .next()
            .ok_or("Expected query end index")?
            .parse::<usize>()
            .map_err(|_| "Invalid query end index")?;
        queries.push((l, r));
    }
    
    // Call the function to check if the array can be zeroed.
    if is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }
    
    Ok(())
}