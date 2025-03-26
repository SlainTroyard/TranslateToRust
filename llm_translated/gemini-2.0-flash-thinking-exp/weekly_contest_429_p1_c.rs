use std::io;

// Function to calculate the minimum operations as per the original C code logic.
fn minimum_operations(nums: &[i32]) -> i32 {
    let mut count = [0; 101]; // Array to store counts of numbers, same size as in C
    for i in (0..nums.len()).rev() { // Iterate in reverse order, same as in C
        count[nums[i] as usize] += 1; // Increment count for the current number
        if count[nums[i] as usize] > 1 { // Check if count is greater than 1 (duplicate found)
            return (i as i32 + 3) / 3; // Return the calculated value, same formula as in C
        }
    }
    0 // Return 0 if no duplicate is found
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line"); // Read input line for n
    let n: i32 = n_str.trim().parse().expect("Invalid input for n"); // Parse n from string to i32

    let mut nums: Vec<i32> = Vec::with_capacity(n as usize); // Initialize a vector to store numbers
    for _ in 0..n { // Loop n times to read numbers
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("Failed to read line"); // Read input line for a number
        let num: i32 = num_str.trim().parse().expect("Invalid input for number"); // Parse number from string to i32
        nums.push(num); // Add the number to the vector
    }

    let result = minimum_operations(&nums); // Call the minimum_operations function
    println!("{}", result); // Print the result to stdout
}