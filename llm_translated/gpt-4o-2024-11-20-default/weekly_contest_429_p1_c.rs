use std::io;

fn minimum_operations(nums: &[i32]) -> i32 {
    // Create an array to count occurrences of numbers from 0 to 100
    let mut count = [0; 101];
    let nums_size = nums.len();

    // Iterate over the numbers in reverse
    for i in (0..nums_size).rev() {
        count[nums[i] as usize] += 1;

        // If any number is encountered more than once, perform the operation calculation
        if count[nums[i] as usize] > 1 {
            return ((i as i32) + 3) / 3;
        }
    }

    // If no repeated number is found, return 0
    0
}

fn main() {
    let mut input = String::new();

    // Read the input size
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to parse input size");

    // Initialize the vector to hold the input numbers
    let mut nums = Vec::with_capacity(n);

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the numbers from the input line
    nums.extend(
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Failed to parse number")),
    );

    // Validate we received the correct number of elements
    assert_eq!(nums.len(), n, "The number of elements does not match the input size");

    // Call the function and print the result
    let result = minimum_operations(&nums);
    println!("{}", result);
}