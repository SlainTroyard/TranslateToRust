use std::io::{self, Write};

// Function to find the k-th character in the sequence
fn kth_character(k: usize) -> char {
    // Allocate a vector to store the counts, initialized to 0
    let mut l = vec![0; k + 1];
    
    // Iterate over the levels of the sequence
    for a in 0.. {
        // Calculate the starting index for the current level
        let j = 1 << a;
        
        // Iterate over the elements in the current level
        for i in 0..(1 << a) {
            // Update the count for the current element
            l[i + j] = l[i] + 1;
            
            // If we have reached or passed the k-th element, return the corresponding character
            if i + j >= k - 1 {
                return (97 + l[k - 1] % 26) as u8 as char;
            }
        }
    }
    
    // This line is unreachable, but Rust requires a return value
    'a'
}

fn main() {
    // Read the input value for k
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let k: usize = input.trim().parse().expect("Invalid input");
    
    // Compute and print the k-th character
    let result = kth_character(k);
    println!("{}", result);
}