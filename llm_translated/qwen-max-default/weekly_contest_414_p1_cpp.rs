use std::io::{self, Write};

/// Converts a decimal number to its binary representation as a string,
/// removing leading zeros.
fn to_binary_string(mut num: u32) -> String {
    if num == 0 {
        return "0".to_string();
    }
    let mut binary = String::new();
    while num > 0 {
        binary.insert(0, if num & 1 == 1 { '1' } else { '0' });
        num >>= 1;
    }
    binary
}

struct Solution;

impl Solution {
    /// Converts a date in the format "YYYY-MM-DD" to a binary representation.
    fn convert_date_to_binary(date: &str) -> String {
        let year = to_binary_string(date[0..4].parse::<u32>().unwrap());
        let month = to_binary_string(date[5..7].parse::<u32>().unwrap());
        let day = to_binary_string(date[8..10].parse::<u32>().unwrap());

        format!("{}-{}-{}", year, month, day)
    }
}

fn main() {
    // Read the input date from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();  // Trim any trailing whitespace

    // Create an instance of the Solution struct and call the method
    let result = Solution::convert_date_to_binary(input);

    // Print the result to stdout
    println!("{}", result);
}