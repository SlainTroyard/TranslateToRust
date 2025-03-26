use std::io::{self, Read};

fn is_zero_array(nums: &[i32], queries: &[[i32; 2]]) -> bool {
    let n = nums.len();
    let mut diff = vec![0; n];
    
    // Process each query to build the difference array
    for &[start, end] in queries {
        let start = start as usize;
        diff[start] += 1;
        let r = end as usize;
        if r + 1 < n {
            diff[r + 1] -= 1;
        }
    }
    
    // Check if all elements can be reduced to zero
    let mut count = 0;
    for (i, &num) in nums.iter().enumerate() {
        count += diff[i];
        if num > count {
            return false;
        }
    }
    true
}

fn main() {
    // Read all input into a string and parse into integers
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let mut cursor = 0;
    
    // Parse nums array
    let nums_size = numbers[cursor] as usize;
    cursor += 1;
    let nums = &numbers[cursor..cursor + nums_size];
    cursor += nums_size;
    
    // Parse queries
    let queries_size = numbers[cursor] as usize;
    cursor += 1;
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let start = numbers[cursor];
        let end = numbers[cursor + 1];
        queries.push([start, end]);
        cursor += 2;
    }
    
    // Check and print result
    println!("{}", if is_zero_array(nums, &queries) { "true" } else { "false" });
}