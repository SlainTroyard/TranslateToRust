use std::io::{self, Read};

fn construct_transformed_array(nums: &[i32]) -> Vec<i32> {
    let nums_size = nums.len();
    let mut result = vec![0; nums_size];
    for (i, &num) in nums.iter().enumerate() {
        if num == 0 {
            result[i] = 0;
        } else {
            let steps = num;
            let sum = i as i32 + steps;
            // Use Euclidean modulo to handle negative values correctly
            let target_index = sum.rem_euclid(nums_size as i32) as usize;
            result[i] = nums[target_index];
        }
    }
    result
}

fn main() {
    // Read entire input and split into whitespace-separated tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Parse array size from first token
    let nums_size: usize = tokens
        .next()
        .and_then(|s| s.parse().ok())
        .expect("Invalid array size");

    // Parse exactly nums_size integers for the array
    let nums: Vec<i32> = tokens
        .by_ref()
        .take(nums_size)
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();

    // Verify we got the correct number of elements
    assert_eq!(nums.len(), nums_size, "Incorrect number of array elements");

    // Generate transformed array
    let transformed = construct_transformed_array(&nums);

    // Format output with space-separated values and trailing newline
    let output = transformed
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", output);
}