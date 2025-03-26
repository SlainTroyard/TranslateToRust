use std::io::{self, BufRead};

// Function to recursively search for the k-th character
fn kchar_search(k: i64, operations: &[i32], pos: usize) -> char {
    let mut pow_sum = 1;
    let mut tmp_pos = 0;

    // Base case: if pos is 0 or k is 1
    if pos == 0 || k == 1 {
        return if operations[pos] != 0 { 'b' } else { 'a' };
    }

    // Calculate the power of 2 that is just greater than k
    while k > pow_sum {
        pow_sum *= 2;
        tmp_pos += 1;
    }

    // If the operation at pos is 1, increment the character
    if operations[pos] != 0 {
        let mut kchar = kchar_search(k - pow_sum / 2, operations, tmp_pos - 1);
        kchar = ((kchar as u8) + 1) as char;
        if kchar > 'z' {
            return 'a';
        }
        return kchar;
    }

    // Otherwise, continue searching
    kchar_search(k - pow_sum / 2, operations, tmp_pos - 1)
}

// Function to find the k-th character
fn kth_character(k: i64, operations: &[i32]) -> char {
    let mut pow_sum = 1;
    let mut pos = 0;

    // Base case: if k is 1, return 'a'
    if k == 1 {
        return 'a';
    }

    // Calculate the power of 2 that is just greater than k
    while pow_sum < k {
        pow_sum *= 2;
        pos += 1;
    }

    // Start the recursive search
    kchar_search(k - pow_sum / 2, operations, pos - 1)
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

    // Find and print the k-th character
    let result = kth_character(k, &operations);
    println!("{}", result);
}