use std::io::{self, Write};

struct Solution;

impl Solution {
    // Function to convert a date string to its binary representation
    pub fn convert_date_to_binary(date: &str) -> String {
        // Helper closure to convert an integer to its binary representation without leading zeros
        let bin = |x: i32| -> String {
            let binary = format!("{:b}", x); // Convert to binary string
            binary
        };

        // Parse the year, month, and day from the input date string
        let year = date[0..4].parse::<i32>().expect("Invalid year format");
        let month = date[5..7].parse::<i32>().expect("Invalid month format");
        let day = date[8..10].parse::<i32>().expect("Invalid day format");

        // Convert each part to binary and concatenate with dashes
        format!("{}-{}-{}", bin(year), bin(month), bin(day))
    }
}

fn main() {
    // Create a buffer for reading input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let date = input.trim(); // Remove any trailing newline or whitespace

    // Create an instance of the solution and process the input
    let sol = Solution;
    let result = sol.convert_date_to_binary(date);

    // Print the result to stdout
    println!("{}", result);
}