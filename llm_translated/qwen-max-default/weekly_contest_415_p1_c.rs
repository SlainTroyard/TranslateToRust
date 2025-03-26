use std::io::{self, BufRead};

fn get_sneaky_numbers(nums: &[i32]) -> [i32; 2] {
    let mut result = [0; 2];
    let mut count = 0;
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                result[count] = nums[i];
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
    // Read the number of elements from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_size: usize = input.trim().parse().expect("Please type a number!");

    // Adjust the size as per the original C code
    let num_size = num_size + 2;

    // Allocate and read the numbers
    let mut nums = vec![0; num_size];
    for i in 0..num_size {
        input.clear(); // Clear the buffer for the next read
        io::stdin().read_line(&mut input).expect("Failed to read line");
        nums[i] = input.trim().parse().expect("Please type a number!");
    }

    // Get the sneaky numbers
    let result = get_sneaky_numbers(&nums);

    // Print the result
    for &num in result.iter() {
        print!("{} ", num);
    }
}