use std::io::{self, Write};

// Recursive function to find the k-th character
fn kchar_search(k: i64, operations: &[i32], pos: usize) -> char {
    let mut pow_sum = 1;
    let mut tmp_pos = 0;

    // Base case: if pos is 0 or k is 1
    if pos == 0 || k == 1 {
        return if operations[pos] != 0 { 'b' } else { 'a' };
    }

    // Find the position where pow_sum is just greater than k
    while k > pow_sum {
        pow_sum *= 2;
        tmp_pos += 1;
    }

    // If the operation is 1, increment the character
    if operations[pos] != 0 {
        let mut kchar = kchar_search(k - pow_sum / 2, operations, tmp_pos - 1);
        kchar = ((kchar as u8) + 1) as char;
        if kchar > 'z' {
            return 'a';
        }
        return kchar;
    }

    // Otherwise, return the character without incrementing
    kchar_search(k - pow_sum / 2, operations, tmp_pos - 1)
}

// Function to find the k-th character based on operations
fn kth_character(k: i64, operations: &[i32]) -> char {
    let mut pow_sum = 1;
    let mut pos = 0;

    // Base case: if k is 1
    if k == 1 {
        return 'a';
    }

    // Find the position where pow_sum is just greater than k
    while pow_sum < k {
        pow_sum *= 2;
        pos += 1;
    }

    // Call the recursive function to find the character
    kchar_search(k - pow_sum / 2, operations, pos - 1)
}

fn main() {
    // Read input values
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input for k");
    let operation_size: usize = iter.next().unwrap().parse().expect("Invalid input for operation size");

    // Read operations array
    let mut operations = Vec::with_capacity(operation_size);
    for _ in 0..operation_size {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let op: i32 = input.trim().parse().expect("Invalid input for operation");
        operations.push(op);
    }

    // Find and print the k-th character
    let result = kth_character(k, &operations);
    println!("{}", result);
}