use std::io::{self, BufRead};
use std::iter::FromIterator;

fn is_zero_array(nums: &[i32], queries: &[(usize, usize)]) -> bool {
    let nums_size = nums.len();
    let mut diff = vec![0; nums_size];
    let mut count = 0;

    // Apply range updates using the difference array technique
    for &(l, r) in queries {
        diff[l] += 1;
        if r + 1 < nums_size {
            diff[r + 1] -= 1;
        }
    }

    // Check if the array can be zeroed
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

    // Read the size of the `nums` array
    let nums_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse numsSize");

    // Read the `nums` array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse element of nums"))
        .collect();

    assert_eq!(nums.len(), nums_size, "nums array size mismatch");

    // Read the number of queries
    let queries_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse queriesSize");

    // Read the queries
    let mut queries = Vec::new();
    for _ in 0..queries_size {
        let query: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse query element"))
            .collect();

        assert_eq!(query.len(), 2, "Each query must have exactly two elements");
        queries.push((query[0], query[1]));
    }

    // Call the function and output the result
    if is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }
}