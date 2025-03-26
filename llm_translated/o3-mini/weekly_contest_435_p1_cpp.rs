use std::io::{self, BufRead};

fn max_difference(s: &str) -> i32 {
    // Create an array of 26 integers initialized to zero for counts of each letter
    let mut cnt = [0; 26];
    
    // Count frequency of each character in the string
    // Since input is lowercase, 'a' corresponds to index 0
    for b in s.chars() {
        // Safe subtraction: assuming input only contains lowercase letters
        cnt[(b as u8 - b'a') as usize] += 1;
    }

    // Initialize max1 for maximum odd frequency and min0 for minimum even frequency
    let mut max1 = 0;
    // Using i32::MAX equivalent to C++'s INT_MAX
    let mut min0 = i32::MAX;

    // Process each frequency in the count array
    for &c in &cnt {
        // Only consider non-zero counts
        if c % 2 == 1 {
            max1 = max1.max(c);
        } else if c != 0 {
            min0 = min0.min(c);
        }
    }
    // Return the difference as required by the problem
    max1 - min0
}

fn main() -> io::Result<()> {
    // Create a buffered reader for standard input
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    
    // Read the input.
    // The original C++ code uses "cin >> s", which reads one token from stdin.
    // Here, we use split_whitespace() to mimic that behavior.
    reader.read_to_string(&mut input)?;
    // Split the input by whitespace and take the first token.
    let token = input.split_whitespace().next().unwrap_or("");

    // Calculate the result using our max_difference function
    let result = max_difference(token);

    // Print the result to stdout.
    println!("{}", result);
    Ok(())
}