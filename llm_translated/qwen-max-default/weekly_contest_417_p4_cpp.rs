// Problem: Weekly Contest 417 Problem 4

use std::io::{self, BufRead};

fn kth_character(k: u64, operations: &Vec<i32>) -> char {
    let mut k = k - 1;
    let mut inc = 0;
    for i in (0..64).rev() {
        if (k >> i) & 1 == 1 {
            if i < operations.len() as u32 {
                inc += operations[i as usize];
            }
        }
    }
    ('a' as u8 + (inc % 26) as u8) as char
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line containing k and operationSize
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let k: u64 = iter.next().unwrap().parse().unwrap();
    let operation_size: usize = iter.next().unwrap().parse().unwrap();

    // Read the second line containing the operations
    let operations_line = lines.next().unwrap().unwrap();
    let operations: Vec<i32> = operations_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Create an instance of the Solution struct and call kthCharacter
    let result = kth_character(k, &operations);

    // Print the result
    println!("{}", result);
}