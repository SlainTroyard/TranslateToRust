// Problem: Weekly Contest 424 Problem 3
use std::io;

fn min_zero_array(nums: &[i32], queries: &Vec<Vec<i32>>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    if queries.is_empty() {
        return -1;
    }

    let nums_size = nums.len();
    let queries_size = queries.len();
    let mut max_diff = vec![0; nums_size + 1];

    let mut sum = 0;
    let mut k = 0;
    for i in 0..nums_size {
        while sum + max_diff[i] < nums[i] {
            if k == queries_size {
                return -1;
            }

            let query = &queries[k];
            let start = query[0] as usize;
            let end = query[1] as usize;
            let increment = query[2];
            k += 1;

            if i > end {
                continue;
            }

            if i >= start {
                max_diff[i] += increment;
            } else {
                max_diff[start] += increment;
            }
            if (end + 1) < max_diff.len() { // Check boundary before accessing
                max_diff[end + 1] -= increment;
            }
        }
        sum += max_diff[i];
    }

    k as i32
}

fn main() {
    // Reading input for nums
    let mut nums_size_str = String::new();
    io::stdin().read_line(&mut nums_size_str).unwrap();
    let nums_size: usize = nums_size_str.trim().parse().unwrap();

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).unwrap();
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Reading input for queries
    let mut queries_size_str = String::new();
    io::stdin().read_line(&mut queries_size_str).unwrap();
    let queries_size: usize = queries_size_str.trim().parse().unwrap();

    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let mut query_str = String::new();
        io::stdin().read_line(&mut query_str).unwrap();
        let query: Vec<i32> = query_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }

    // Calling the function
    let result = min_zero_array(&nums, &queries);

    // Output the result
    println!("{}", result);
}