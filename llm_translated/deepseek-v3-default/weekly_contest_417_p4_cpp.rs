use std::io::{self, BufRead};

fn kth_character(k: i64, operations: &[i32]) -> char {
    let mut k = k - 1;
    let mut inc = 0;
    for i in (0..=63).rev() { // Equivalent to __lg(k) in C++
        if (k >> i) & 1 == 1 {
            inc += operations[i as usize];
        }
    }
    (b'a' + (inc % 26) as u8) as char
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line containing k and operationSize
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
    
    // Compute and print the result
    let result = kth_character(k, &operations);
    println!("{}", result);
}