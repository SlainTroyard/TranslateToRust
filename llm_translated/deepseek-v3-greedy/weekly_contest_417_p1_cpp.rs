use std::io;

// Solution struct to encapsulate the logic
struct Solution;

impl Solution {
    // Function to find the kth character
    fn kth_character(k: i64) -> char {
        // Calculate the number of set bits in (k - 1) and add it to 'a'
        ('a' as u8 + (k - 1).count_ones() as u8) as char
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse the input as an integer
    let k: i64 = input.trim().parse().expect("Input is not a valid integer");

    // Create an instance of Solution and call the function
    let result = Solution::kth_character(k);

    // Print the result
    println!("{}", result);
}