use std::io::{self, BufRead};

/// Checks if an array can be zeroed by applying a series of queries.
///
/// Each query [l, r] decrements all elements from index l to r (inclusive).
/// The function returns true if all elements can be reduced to zero or less.
fn is_zero_array(nums: &[i32], queries: &[Vec<i32>]) -> bool {
    let nums_size = nums.len();
    let mut diff = vec![0; nums_size];
    let mut count = 0;

    // Apply the difference array technique to efficiently track the effect of queries
    for query in queries {
        let l = query[0] as usize;
        let r = query[1] as usize;
        
        diff[l] += 1;
        if (r + 1) < nums_size {
            diff[r + 1] -= 1;
        }
    }

    // Check if each element can be reduced to zero or less
    for i in 0..nums_size {
        count += diff[i];
        if nums[i] > count {
            return false;
        }
    }
    
    true
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read input for nums array size and elements
    let nums_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let nums: Vec<i32> = lines.next().unwrap()?.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read input for queries size and elements
    let queries_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut queries = Vec::with_capacity(queries_size);
    
    for _ in 0..queries_size {
        let query_line = lines.next().unwrap()?;
        let query: Vec<i32> = query_line.trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }
    
    // Call the function to check if the array can be zeroed
    if is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }
    
    Ok(())
}