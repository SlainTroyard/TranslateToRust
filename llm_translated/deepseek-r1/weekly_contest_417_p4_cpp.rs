use std::io::{self, BufRead};

fn kth_character(mut k: i64, operations: &[i32]) -> char {
    k -= 1; // Convert to 0-based index as in the original code
    let mut inc = 0;

    if k != 0 {
        // Calculate the highest set bit position using leading zeros
        let highest_bit = (63 - k.leading_zeros() as i32) as i32;
        // Iterate from highest bit down to 0, same as __lg in C++
        for i in (0..=highest_bit).rev() {
            if (k >> i) & 1 != 0 {
                inc += operations[i as usize];
            }
        }
    }

    // Compute the resulting character with modulo 26
    let c = b'a' + (inc % 26) as u8;
    c as char
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read first line: k and operation size
    let first_line = lines.next().expect("Failed to read first line").unwrap();
    let mut parts = first_line.split_whitespace();
    let k: i64 = parts.next().unwrap().parse().expect("Invalid k");
    let operation_size: usize = parts.next().unwrap().parse().expect("Invalid operation size");

    // Read second line: operation values
    let second_line = lines.next().expect("Failed to read second line").unwrap();
    let operations: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid operation"))
        .collect();

    // Ensure the operation count matches the input size
    assert_eq!(operations.len(), operation_size);

    // Compute and print the result
    let result = kth_character(k, &operations);
    println!("{}", result);
}