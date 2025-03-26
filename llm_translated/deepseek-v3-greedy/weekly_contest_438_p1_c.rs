use std::io::{self, Write};

fn has_same_digits(s: &mut Vec<u8>) -> bool {
    let len = s.len();
    let mut i = 0;
    
    // Convert each character to its numeric value by masking with 0x0f
    while i < len {
        s[i] &= 0x0f;
        i += 1;
    }
    
    // Perform the reduction step until the length is 2
    let mut current_len = len;
    while current_len > 2 {
        for i in 0..current_len - 1 {
            s[i] = (s[i] + s[i + 1]) % 10;
        }
        current_len -= 1;
    }
    
    // Check if the two remaining digits are the same
    s[0] == s[1]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Remove any trailing newline characters
    let input = input.trim();
    
    // Convert the input string to a mutable vector of bytes
    let mut s = input.bytes().collect::<Vec<u8>>();
    
    // Check if the digits are the same and print the result
    println!("{}", has_same_digits(&mut s));
}