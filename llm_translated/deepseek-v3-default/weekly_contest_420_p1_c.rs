use std::io::{self, Write};

fn string_sequence(target: &str) -> Vec<String> {
    // Calculate the total number of sequences
    let return_size: usize = target.chars().map(|c| (c as u8 - b'a' + 1) as usize).sum();
    
    // Initialize the result vector
    let mut ans: Vec<String> = Vec::with_capacity(return_size);
    
    // Iterate over each character in the target string
    for (l, t) in target.chars().enumerate() {
        // Generate sequences from 'a' to the current character
        for c in ('a'..=t).into_iter() {
            // Create a new string by copying the prefix and appending the current character
            let mut sequence = String::with_capacity(l + 1);
            sequence.push_str(&target[..l]);
            sequence.push(c);
            ans.push(sequence);
        }
    }
    
    ans
}

fn main() {
    // Read the input string
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read input");
    target = target.trim().to_string(); // Remove any trailing newline
    
    // Generate the sequence
    let ans = string_sequence(&target);
    
    // Print the sequences separated by spaces
    for sequence in &ans {
        print!("{} ", sequence);
    }
    println!(); // Print a newline at the end
}