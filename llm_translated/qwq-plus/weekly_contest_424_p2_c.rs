use std::io;

/// Checks if the array can be zeroed after applying all queries.
///
/// # Arguments
/// * `nums` - The array of integers to check.
/// * `queries` - A list of (start, end) pairs representing ranges to increment.
///
/// # Returns
/// `true` if the array can be zeroed, `false` otherwise.
fn is_zero_array(nums: &[i32], queries: &[(usize, usize)]) -> bool {
    let n = nums.len();
    let mut diff = vec![0; n];

    // Apply all queries to the difference array
    for &(start, end) in queries {
        diff[start] += 1;
        if end + 1 < n {
            diff[end + 1] -= 1;
        }
    }

    let mut count = 0;
    for (i, &d) in diff.iter().enumerate() {
        count += d;
        // If current element exceeds the cumulative count, it's impossible to zero
        if nums[i] > count {
            return false;
        }
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Split input into tokens and parse them as integers
    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse integer"));

    // Read nums array
    let nums_size = tokens.next().unwrap() as usize;
    let nums: Vec<i32> = (0..nums_size)
        .map(|_| tokens.next().unwrap())
        .collect();

    // Read queries
    let queries_size = tokens.next().unwrap() as usize;
    let queries: Vec<(usize, usize)> = (0..queries_size)
        .map(|_| {
            let a = tokens.next().unwrap() as usize;
            let b = tokens.next().unwrap() as usize;
            (a, b)
        })
        .collect();

    // Determine and print the result
    println!("{}", if is_zero_array(&nums, &queries) { "true" } else { "false" });
}