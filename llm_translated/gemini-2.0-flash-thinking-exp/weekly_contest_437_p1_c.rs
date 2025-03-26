fn has_special_substring(s: &str, k: i32) -> bool {
    // Get the length of the input string
    let length = s.len();
    // Initialize a counter for consecutive identical characters
    let mut consecutive = 0;
    // Convert the string to a vector of characters for easy indexing
    let chars: Vec<char> = s.chars().collect();

    // Iterate through the characters of the string
    for i in 0..length {
        // Increment the consecutive counter for each character
        consecutive += 1;
        // Check if it's the last character or if the current character is different from the next one
        if i == length - 1 || chars[i] != chars[i + 1] {
            // If the consecutive count is equal to k, a special substring is found
            if consecutive == k {
                return true; // Return true immediately
            } else {
                // Otherwise, reset the consecutive counter to 0
                consecutive = 0;
            }
        }
    }
    // If the loop finishes without finding a special substring, return false
    false
}

fn main() {
    use std::io;

    // Read a line from standard input
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");

    // Split the input line by whitespace to separate the string and integer
    let parts: Vec<&str> = line.trim().split_whitespace().collect();

    // Extract the input string (the first part)
    let input_s = parts[0];
    // Extract the integer k (the second part) and parse it from string to i32
    let k: i32 = parts[1].parse().expect("Please type a number!");

    // Call the has_special_substring function to check for the special substring
    let result = has_special_substring(input_s, k);

    // Print 1 if a special substring is found, 0 otherwise, as per the original C code output format
    println!("{}", if result { 1 } else { 0 });
}