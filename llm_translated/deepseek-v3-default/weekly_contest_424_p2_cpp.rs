use std::io;

fn main() {
    // Read the size of the nums array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input for n");

    // Read the nums array
    let mut nums = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    for num_str in input.split_whitespace() {
        let num: i32 = num_str.parse().expect("Invalid number in nums array");
        nums.push(num);
    }

    // Read the number of queries
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let m: usize = input.trim().parse().expect("Invalid input for m");

    // Read the queries
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let query: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid query value"))
            .collect();
        queries.push(query);
    }

    // Call the solution function and print the result
    if is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }
}

fn is_zero_array(nums: &[i32], queries: &[Vec<usize>]) -> bool {
    let mut v = vec![0; nums.len() + 1];

    // Apply the queries to the difference array
    for query in queries {
        let start = query[0];
        let end = query[1];
        v[start] += 1;
        if end + 1 < v.len() {
            v[end + 1] -= 1;
        }
    }

    // Compute the prefix sum to get the actual counts
    for i in 1..nums.len() {
        v[i] += v[i - 1];
    }

    // Check if all elements in nums can be zeroed
    for i in 0..nums.len() {
        if nums[i] - v[i] > 0 {
            return false;
        }
    }

    true
}