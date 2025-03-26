use std::io::{self, Read};

fn get_sneaky_numbers(nums: &[i32]) -> Vec<i32> {
    let mut result = vec![0; 2]; // Initialize with two zeros as in C's calloc
    let mut count = 0;
    'outer: for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                result[count] = nums[i];
                count += 1;
                if count == 2 {
                    break 'outer;
                }
                break; // Break inner loop to check next i
            }
        }
    }
    result
}

fn main() {
    // Read entire input as a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Parse the initial number which determines the input size (original_num_size)
    let original_num_size: i32 = tokens.next()
        .expect("Missing initial size")
        .parse()
        .expect("Invalid number format");
    let actual_num_size = original_num_size + 2;

    // Collect exactly (original_num_size + 2) numbers as per problem requirements
    let nums: Vec<i32> = tokens
        .take(actual_num_size as usize)
        .map(|s| s.parse().expect("Invalid number in input"))
        .collect();

    // Ensure we have the exact number of elements specified by the problem
    assert_eq!(
        nums.len(),
        actual_num_size as usize,
        "Input does not contain enough numbers"
    );

    // Get the sneaky numbers and print them with exact original format
    let result = get_sneaky_numbers(&nums);
    for num in result {
        print!("{} ", num);
    }
}