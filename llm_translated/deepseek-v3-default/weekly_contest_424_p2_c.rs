use std::io::{self, BufRead};

fn is_zero_array(nums: &[i32], queries: &[[usize; 2]]) -> bool {
    let mut diff = vec![0; nums.len()];
    let mut count = 0;

    for &[start, end] in queries {
        diff[start] += 1;
        if end + 1 < nums.len() {
            diff[end + 1] -= 1;
        }
    }

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

    // Read input for nums array size and elements
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read input for queries size and elements
    let queries_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut queries = Vec::with_capacity(queries_size);

    for _ in 0..queries_size {
        let query_line = lines.next().unwrap().unwrap();
        let mut parts = query_line.split_whitespace();
        let start: usize = parts.next().unwrap().parse().unwrap();
        let end: usize = parts.next().unwrap().parse().unwrap();
        queries.push([start, end]);
    }

    // Call the function to check if the array can be zeroed
    if is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }
}