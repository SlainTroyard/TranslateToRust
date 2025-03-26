use std::io::{self, BufRead};

fn is_zero_array(nums: &[i32], queries: &[[usize; 2]]) -> bool {
    let mut diff = vec![0; nums.len()];
    let mut count = 0;

    // Apply the queries to the difference array
    for &[start, end] in queries {
        diff[start] += 1;
        if end + 1 < nums.len() {
            diff[end + 1] -= 1;
        }
    }

    // Check if the array can be zeroed
    for i in 0..nums.len() {
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

    // Read the size of the nums array
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the nums array
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read the size of the queries array
    let queries_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let query: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push([query[0], query[1]]);
    }

    // Check if the array can be zeroed
    if is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }
}