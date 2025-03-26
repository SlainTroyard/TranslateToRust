use std::io;

// Function to count the number of set bits in a number
fn count_set_bits(mut n: u64) -> u8 {
    let mut count = 0;
    while n != 0 {
        count += (n & 1) as u8;
        n >>= 1;
    }
    count
}

// Function to find the kth character
fn kth_character(k: u64) -> char {
    // The character is 'a' plus the number of set bits in (k - 1)
    ('a' as u8 + count_set_bits(k - 1)) as char
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as a u64
    let k: u64 = input.trim().parse().expect("Please enter a valid number");
    
    // Compute and print the kth character
    println!("{}", kth_character(k));
}