// Problem: Weekly Contest 417 Problem 1
use std::io;

fn kth_character(k: usize) -> char {
    // Create a vector with k+1 elements initialized to 0
    let mut l = vec![0; k + 1];
    
    let mut a = 0;
    loop {
        let j = 1 << a; // 2^a
        
        for i in 0..(1 << a) { // Loop from 0 to 2^a - 1
            l[i + j] = l[i] + 1;
            
            if i + j >= k - 1 {
                // Convert to ASCII character: 'a' + (l[k-1] % 26)
                return char::from_u32(97 + (l[k - 1] % 26) as u32).unwrap();
            }
        }
        
        a += 1;
    }
}

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    // Parse input to integer
    let k: usize = input
        .trim()
        .parse()
        .expect("Failed to parse input as integer");
    
    // Get the kth character and print it
    println!("{}", kth_character(k));
}