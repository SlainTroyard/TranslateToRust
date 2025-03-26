use std::io::{self, BufRead, Write};

/// Calculate the score based on the given string.
/// This function follows the same logic as the original C code.
/// It uses one stack (implemented as a Vec<usize>) for each letter.
fn calculate_score(s: &str) -> i64 {
    // Get the length of the string.
    let len = s.len();
    
    // Create 26 stacks (one per letter) with capacity equal to the length of the string.
    // Using std::array::from_fn to create an array; requires Rust 1.63+.
    let mut stacks: [Vec<usize>; 26] = std::array::from_fn(|_| Vec::with_capacity(len));

    // Accumulator for the final answer.
    let mut ans: i64 = 0;

    // Iterate over the characters of the string, along with their indices.
    // We're assuming that the input string contains only lowercase ASCII letters.
    for (i, ch) in s.chars().enumerate() {
        // Convert the character to its index (0 for 'a', 25 for 'z').
        let c = (ch as u8 - b'a') as usize;
        // Check the corresponding "matching" stack indexed at 25 - c.
        if let Some(&top_pos) = stacks[25 - c].last() {
            // Found a match: pop the top element and calculate the score difference.
            stacks[25 - c].pop();
            ans += i as i64 - top_pos as i64;
        } else {
            // No match found: push the current index onto the stack for this letter.
            stacks[c].push(i);
        }
    }
    
    // Return the computed answer.
    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use standard input to read the entire input.
    // The original code uses scanf("%s", s), which reads a single token (delimited by whitespace).
    // Here, we read from stdin and extract the first token using whitespace splitting.
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // Read the first non-empty line. If the input format is such that the string is provided
    // on one line (or among multiple lines, the first token is relevant), we pick the first token.
    let mut input_string = String::new();
    while let Some(line) = iterator.next() {
        let line = line?;
        // Split the line on whitespace and take the first token if available.
        if let Some(token) = line.split_whitespace().next() {
            input_string = token.to_string();
            break;
        }
    }
    
    // Calculate the score with the provided string.
    let result = calculate_score(&input_string);
    
    // Output the result with exact same format as the original C code.
    // Using println! to print the result followed by a newline.
    println!("{}", result);
    
    // Ensure proper flushing of output.
    io::stdout().flush()?;
    
    Ok(())
}