use std::io;

fn main() {
    // Read the date from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();  // Trim any whitespace/newline characters

    // Create an instance of Solution and call convert_date_to_binary
    let sol = Solution;
    let result = sol.convert_date_to_binary(input);
    println!("{}", result);
}

struct Solution;

impl Solution {
    /// Converts a given date in "YYYY-MM-DD" format to its binary representation.
    fn convert_date_to_binary(&self, date: &str) -> String {
        // Helper function to convert an integer to its binary string representation
        let bin = |x: i32| -> String {
            format!("{:b}", x)
        };

        // Extract year, month, and day from the date string
        let year = date[0..4].parse::<i32>().unwrap();
        let month = date[5..7].parse::<i32>().unwrap();
        let day = date[8..10].parse::<i32>().unwrap();

        // Convert each part to binary and concatenate with dashes
        format!("{}-{}-{}", bin(year), bin(month), bin(day))
    }
}