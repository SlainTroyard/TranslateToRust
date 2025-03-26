use std::io::{self, BufRead};

/// Recursive helper function to find the k-th character.
/// Translated from the `kchar_search` function in the C code.
fn kchar_search(k: i64, operations: &[i32], pos: usize) -> char {
    let mut pow_sum = 1;
    let mut tmp_pos = 0;

    // Base cases
    if pos == 0 || k == 1 {
        return if operations[pos] != 0 { 'b' } else { 'a' };
    }

    // Identify the position in the doubling sequence
    while k > pow_sum {
        pow_sum *= 2;
        tmp_pos += 1;
    }

    if operations[pos] != 0 {
        let mut kchar = kchar_search(k - pow_sum / 2, operations, tmp_pos - 1);
        kchar = ((kchar as u8 + 1 - b'a') % 26 + b'a') as char; // Increment and wrap around if needed
        return kchar;
    }

    kchar_search(k - pow_sum / 2, operations, tmp_pos - 1)
}

/// Function to find the k-th character based on the operations.
/// Translated from the `kthCharacter` function in the C code.
fn kth_character(k: i64, operations: &[i32]) -> char {
    let mut pow_sum = 1;
    let mut pos = 0;

    // Handle base case
    if k == 1 {
        return 'a';
    }

    // Identify the position in the doubling sequence
    while pow_sum < k {
        pow_sum *= 2;
        pos += 1;
    }

    // Recursively find the k-th character
    kchar_search(k - pow_sum / 2, operations, pos - 1)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse `k` and `operationsSize`
    let first_line = lines.next().unwrap().unwrap();
    let mut split = first_line.split_whitespace();
    let k: i64 = split.next().unwrap().parse().unwrap();
    let operation_size: usize = split.next().unwrap().parse().unwrap();

    // Parse the `operations` array
    let second_line = lines.next().unwrap().unwrap();
    let operations: Vec<i32> = second_line
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    // Assert that we have the correct number of operations
    assert_eq!(operations.len(), operation_size);

    // Compute and print the result
    let result = kth_character(k, &operations);
    println!("{}", result);
}