use std::io::{self, BufRead};

/// Determines if an array can be reduced to all zeros by applying operations from the queries.
///
/// Each query [l, r] allows decrementing all elements at indices from l to r by 1.
fn is_zero_array(nums: &[i32], queries: &Vec<Vec<i32>>) -> bool {
    let nums_size = nums.len();
    let mut diff = vec![0; nums_size];
    let mut count = 0;

    // Apply the difference array technique to efficiently simulate the operations
    for query in queries {
        let l = query[0] as usize;
        let r = query[1] as usize;
        
        diff[l] += 1;
        if r + 1 < nums_size {
            diff[r + 1] -= 1;
        }
    }

    // Check if we can reduce all elements to zero
    for i in 0..nums_size {
        count += diff[i];
        if nums[i] > count {
            return false;
        }
    }
    
    true
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the nums array size
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the nums array elements
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read the queries size
    let queries_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let query_line = lines.next().unwrap().unwrap();
        let query: Vec<i32> = query_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }
    
    // Check if the array can be zeroed and print the result
    if is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }
}