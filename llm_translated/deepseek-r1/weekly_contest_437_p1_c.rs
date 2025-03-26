use std::io::{self, BufRead};

fn has_special_substring(s: &str, k: usize) -> bool {
    // Convert the string into a vector of characters for easy indexing
    let chars: Vec<char> = s.chars().collect();
    let mut consecutive = 0;

    // Iterate through each character in the string
    for i in 0..chars.len() {
        consecutive += 1;

        // Check if current character is the last or different from the next
        if i == chars.len() - 1 || chars[i] != chars[i + 1] {
            if consecutive == k {
                return true;
            } else {
                consecutive = 0;  // Reset counter for new sequence
            }
        }
    }

    false
}

fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();
    
    // Read input line preserving original C code's input logic
    stdin.lock().read_line(&mut input_line).expect("Failed to read line");
    let mut parts = input_line.trim().split_whitespace();
    
    // Parse string and k value following C's scanf("%s %d") pattern
    let s = parts.next().expect("Missing string in input");
    let k: usize = parts.next()
                       .expect("Missing k value in input")
                       .parse()
                       .expect("k must be an integer");

    // Output 1/0 instead of true/false to match C's boolean printf behavior
    println!("{}", has_special_substring(s, k) as i32);
}