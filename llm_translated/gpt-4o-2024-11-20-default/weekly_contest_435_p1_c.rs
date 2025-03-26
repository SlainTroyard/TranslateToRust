use std::io::{self, Write};

// Function to calculate the maximum difference between the largest odd frequency
// and the smallest even frequency of characters in the string
fn max_difference(s: &str) -> i32 {
    // Initialize the frequency array for 'a' to 'z'
    let mut cnt = [0; 26];
    let mut c1 = 0; // The largest odd frequency
    let mut c2 = 100; // The smallest even frequency

    // Count the frequency of each character in the string
    for ch in s.chars() {
        cnt[ch as usize - 'a' as usize] += 1;
    }

    // Traverse the frequency array to find c1 and c2
    for &frequency in &cnt {
        if frequency & 1 == 1 {
            // If the frequency is odd
            c1 = c1.max(frequency);
        } else if frequency != 0 {
            // If the frequency is even and not zero
            c2 = c2.min(frequency);
        }
    }

    c1 - c2
}

fn main() {
    // Create a buffered reader for input and writer for output
    let mut input = String::new();

    // Read the input string
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Trim the input string to remove any trailing newline or whitespace
    let s = input.trim();

    // Calculate the result
    let result = max_difference(s);

    // Output the result
    println!("{}", result);
}