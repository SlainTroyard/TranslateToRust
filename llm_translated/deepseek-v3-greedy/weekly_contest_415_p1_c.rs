use std::io::{self, Write};

// Function to find the two sneaky numbers in the array
fn get_sneaky_numbers(nums: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(2);
    let mut count = 0;

    // Iterate through the array to find duplicates
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] == nums[j] {
                result.push(nums[i]);
                count += 1;
                break;
            }
        }
        if count == 2 {
            break;
        }
    }

    result
}

fn main() {
    // Read the size of the array from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_size: usize = input.trim().parse().expect("Invalid input");
    let num_size = num_size + 2;

    // Read the array elements from stdin
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Invalid input");
        nums.push(num);
    }

    // Find the sneaky numbers
    let result = get_sneaky_numbers(&nums);

    // Print the result
    for num in result {
        print!("{} ", num);
    }
    io::stdout().flush().expect("Failed to flush stdout");
}