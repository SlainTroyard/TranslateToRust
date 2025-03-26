use std::io;

// Define a struct to hold the solution
struct Solution;

impl Solution {
    // Implement the is_balanced method
    fn is_balanced(num: &str) -> bool {
        let mut even_sum = 0;
        let mut odd_sum = 0;

        // Iterate over the characters of the string
        for (i, c) in num.chars().enumerate() {
            // Convert the character to a digit
            let digit = c.to_digit(10).unwrap();
            // Check if the index is even or odd and add to the corresponding sum
            if i % 2 == 0 {
                even_sum += digit;
            } else {
                odd_sum += digit;
            }
        }

        // Return true if the sums are equal, otherwise false
        even_sum == odd_sum
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // Trim the input to remove any trailing whitespace or newlines
    let num = input.trim();

    // Create a Solution instance and call the is_balanced method
    let result = Solution::is_balanced(num);

    // Print the result in the exact same format as the original code
    if result {
        println!("true");
    } else {
        println!("false");
    }
}