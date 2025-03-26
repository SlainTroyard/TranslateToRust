use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Function to find the k-th character based on the given operations
    fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut k = k - 1; // Adjust k to be zero-based
        let mut inc = 0;

        // Iterate over the bits of k from the highest to the lowest
        for i in (0..64).rev() {
            if (k >> i) & 1 == 1 {
                inc += operations[i as usize];
            }
        }

        // Calculate the character by taking the modulo 26 of the increment and adding it to 'a'
        ((inc % 26) as u8 + b'a') as char
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line containing k and the size of the operations vector
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let k: i64 = parts.next().unwrap().parse().unwrap();
    let operation_size: usize = parts.next().unwrap().parse().unwrap();

    // Read the second line containing the operations
    let second_line = lines.next().unwrap().unwrap();
    let operations: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Ensure the operations vector has the correct size
    assert_eq!(operations.len(), operation_size);

    // Create an instance of Solution and compute the k-th character
    let result = Solution::kth_character(k, operations);

    // Print the result
    println!("{}", result);
}