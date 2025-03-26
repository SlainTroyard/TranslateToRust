use std::io::{self, Write};

fn minimum_operations(nums: &[usize]) -> usize {
    // Array to count occurrences of numbers (0 to 100)
    let mut count = [0; 101];
    
    // Iterate from the end of the array to the beginning
    for i in (0..nums.len()).rev() {
        count[nums[i]] += 1;
        // If a number appears more than once, calculate and return the result
        if count[nums[i]] > 1 {
            return (i + 3) / 3;
        }
    }
    // If no duplicates are found, return 0
    0
}

fn main() {
    // Create a buffer for reading input
    let mut input = String::new();
    // Read the first line (size of the array)
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Failed to parse input as usize");

    // Read the second line (array elements)
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse input as usize"))
        .collect();

    // Ensure the input size matches the declared size
    assert_eq!(nums.len(), n, "Input size does not match the declared size");

    // Call the function and get the result
    let result = minimum_operations(&nums);

    // Print the result
    println!("{}", result);
}