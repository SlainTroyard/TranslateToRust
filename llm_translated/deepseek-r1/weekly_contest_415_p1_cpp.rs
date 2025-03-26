use std::io::{self, Read};

fn main() {
    // Read all input as a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Split into whitespace-separated tokens and parse as integers
    let mut tokens = input.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Invalid input"));

    // Read original_num_size and calculate required vector size
    let original_num_size = tokens.next().expect("No input provided") as usize;
    let num_size = original_num_size + 2;

    // Collect the next num_size numbers into the vector
    let nums: Vec<i32> = tokens.take(num_size).collect();

    // Check if we have enough elements
    if nums.len() != num_size {
        panic!("Insufficient input elements");
    }

    // Process the numbers and get result
    let result = get_sneaky_numbers(nums);

    // Print the result in the required format
    println!("{} {}", result[0], result[1]);
}

fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() - 2;
    let mut xor_all = (n as i32) ^ (n as i32 + 1);

    // XOR all indices and values
    for (i, &num) in nums.iter().enumerate() {
        xor_all ^= (i as i32) ^ num;
    }

    // Find the least significant bit that differs
    let shift = xor_all.trailing_zeros();
    let mut ans = vec![0, 0];

    // Separate numbers into groups based on the significant bit
    for (i, &num) in nums.iter().enumerate() {
        // Process original indices if below n
        if i < n {
            let group = ((i as i32) >> shift) & 1;
            ans[group as usize] ^= i as i32;
        }

        // Process the number itself
        let group = (num >> shift) & 1;
        ans[group as usize] ^= num;
    }

    ans
}