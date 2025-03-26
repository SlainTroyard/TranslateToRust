// Problem: Weekly Contest 417 Problem 1
use std::io;

fn kth_character(k: i32) -> char {
    // Create a vector to store the values, initialized with zeros
    let mut l = vec![0; (k + 1) as usize];
    
    // Iterate through powers of 2
    for a in 0.. {
        let j = 1 << a; // Calculate 2^a
        
        // Iterate through the current segment
        for i in 0..(1 << a) {
            // Calculate the value at position i+j based on the value at position i
            l[i + j] = l[i] + 1;
            
            // If we've reached or passed the target position, return the character
            if (i + j) as i32 >= k - 1 {
                // Convert to ASCII character (97 is 'a')
                return (97 + l[k as usize - 1] % 26) as u8 as char;
            }
        }
    }
    
    // This line is unreachable due to the infinite loop, but Rust requires a return value
    unreachable!();
}

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    // Parse the input as an integer
    let k: i32 = input.trim().parse().expect("Input should be an integer");
    
    // Call the function and print the result
    println!("{}", kth_character(k));
}