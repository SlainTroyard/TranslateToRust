use std::io;
use std::io::BufRead;
use std::vec::Vec;

fn get_sneaky_numbers(nums: &[i32]) -> Vec<i32> {
    // Initialize a vector to store the result with a capacity of 2
    let mut result = Vec::with_capacity(2);
    let mut count = 0;
    let nums_size = nums.len();

    // Iterate through the nums array to find duplicate numbers
    for i in 0..nums_size {
        for j in i + 1..nums_size {
            // Check if nums[i] and nums[j] are equal (duplicate found)
            if nums[i] == nums[j] {
                // Add the duplicate number to the result vector
                result.push(nums[i]);
                count += 1;
                // Break the inner loop after finding the first duplicate for nums[i]
                break;
            }
        }
        // Break the outer loop if we have found two duplicate numbers
        if count == 2 {
            break;
        }
    }
    // Return the result vector
    result
}

fn main() {
    // Read the first line from stdin to get numSize
    let mut num_size_str = String::new();
    io::stdin().read_line(&mut num_size_str).expect("Failed to read line");
    let num_size: i32 = num_size_str.trim().parse().expect("Invalid input for numSize");

    // Calculate the actual number of integers to read (numSize + 2)
    let num_size_plus_two = num_size + 2;

    // Initialize a vector to store the input numbers with the calculated capacity
    let mut nums: Vec<i32> = Vec::with_capacity(num_size_plus_two as usize);

    // Read the second line from stdin which contains space-separated integers
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line = iterator.next().unwrap().unwrap(); // Read the next line and unwrap Result<String, Error> to String, then Result<Option<String>, Error> to Option<String>

    // Split the line by whitespace and parse each part into an i32, then store in the nums vector
    for s in line.split_whitespace() {
        let num: i32 = s.trim().parse().expect("Invalid input for numbers");
        nums.push(num);
    }

    // Call the get_sneaky_numbers function to find the sneaky numbers
    let result = get_sneaky_numbers(&nums);

    // Print the sneaky numbers to stdout, separated by spaces
    for i in 0..result.len() {
        print!("{} ", result[i]);
    }
    println!(); // Add a newline at the end to match the original output format if needed. Though the original C doesn't have newline, keeping it for good practice.
}