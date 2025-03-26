// Problem: Weekly Contest 417 Problem 1
use std::io;

fn kth_character(k: i32) -> char {
    // Create a vector of size k+1 initialized with zeros (equivalent to calloc)
    let mut l = vec![0; (k + 1) as usize];
    let mut a = 0;
    
    loop {
        let j = 1 << a; // 2^a
        for i in 0..(1 << a) {
            // Equivalent to l[i + j] = l[i] + 1
            l[(i + j) as usize] = l[i as usize] + 1;
            
            // Check if we've reached or passed the k-1 index
            if i + j >= k - 1 {
                // Return the character: 'a' (ASCII 97) + l[k-1] % 26
                return (97 + l[(k - 1) as usize] % 26) as u8 as char;
            }
        }
        a += 1;
    }
}

fn main() {
    // Read input value k
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let k: i32 = input.trim().parse().expect("Failed to parse input as integer");
    
    // Output the result
    println!("{}", kth_character(k));
}