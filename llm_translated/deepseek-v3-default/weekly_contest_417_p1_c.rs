use std::io;

// Function to find the k-th character in the sequence
fn kth_character(k: usize) -> char {
    // Allocate a vector to store the levels of the sequence
    let mut l = vec![0; k + 1];
    
    // Iterate through the levels of the sequence
    for a in 0.. {
        let j = 1 << a; // Calculate the starting index for the current level
        
        // Iterate through the elements in the current level
        for i in 0..(1 << a) {
            l[i + j] = l[i] + 1; // Update the level of the current element
            
            // If we have reached or surpassed the k-th element, return the corresponding character
            if i + j >= k - 1 {
                return (97 + l[k - 1] % 26) as u8 as char;
            }
        }
    }
    
    // This line is unreachable, but Rust requires a return value
    '\0'
}

fn main() {
    // Read the input value for k
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: usize = input.trim().parse().expect("Please enter a valid integer");
    
    // Compute and print the k-th character
    println!("{}", kth_character(k));
}