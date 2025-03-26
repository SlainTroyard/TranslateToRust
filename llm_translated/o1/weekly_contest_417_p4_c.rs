// Translated from the original C code for LeetCode Weekly Contest 417 Problem 4
// Requirements:
// 1. Translate the ENTIRE file as a complete program, including the main function.
// 2. Preserve the algorithm logic exactly.
// 3. Use idiomatic Rust with proper error handling.
// 4. Maintain the EXACT SAME stdin/stdout format as the original C code.
// 5. The input is read in the same way (tokens split by whitespace), and the output
//    is a single character followed by a newline.

use std::io::{self, BufRead};

/// Recursively computes the character for the k-th position given the operations.
/// Equivalent to the C function "char kchar_search(long long k, int* operations, int pos)".
fn kchar_search(k: i64, operations: &[i32], pos: usize) -> char {
    let mut pow_sum = 1_i64;
    let mut tmp_pos = 0_usize;

    // If pos == 0 or k == 1, return 'b' if current operation is set, otherwise 'a'.
    if pos == 0 || k == 1 {
        return if operations[pos] != 0 { 'b' } else { 'a' };
    }

    // Find the smallest power of 2 that is >= k.
    while k > pow_sum {
        pow_sum *= 2;
        tmp_pos += 1;
    }

    // If the operation at this position is set, increment the result by 1 if possible.
    if operations[pos] != 0 {
        let c = kchar_search(k - (pow_sum / 2), operations, tmp_pos - 1);
        let next_byte = c as u8 + 1;
        if next_byte > b'z' {
            'a'
        } else {
            next_byte as char
        }
    } else {
        // Otherwise, just recurse without increment.
        kchar_search(k - (pow_sum / 2), operations, tmp_pos - 1)
    }
}

/// Returns the k-th character after all operations. 
/// Equivalent to the C function "char kthCharacter(long long k, int* operations, int operationsSize)".
fn kth_character(k: i64, operations: &[i32]) -> char {
    // Handle the simplest case.
    if k == 1 {
        return 'a';
    }

    // Determine how many times we can double until we exceed or match k.
    let mut pow_sum = 1_i64;
    let mut pos = 0_usize;
    while pow_sum < k {
        pow_sum *= 2;
        pos += 1;
    }

    // Recursively compute the character.
    kchar_search(k - (pow_sum / 2), operations, pos - 1)
}

fn main() -> io::Result<()> {
    // Read all tokens from stdin (whitespace-separated).
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    for line_result in stdin.lock().lines() {
        let line = line_result?;
        for token in line.split_whitespace() {
            tokens.push(token.to_string());
        }
    }

    // Parse the first two tokens: k and operationSize.
    // In the original C code: scanf("%lld %d", &k, &operationSize);
    let k: i64 = tokens[0].parse().expect("Failed to parse k as i64");
    let operation_size: usize = tokens[1].parse().expect("Failed to parse operation_size as usize");

    // Parse the next operation_size tokens as the operations array.
    // Equivalent to the for-loop reading each operation: scanf("%d", &operations[i]);
    let mut operations = Vec::with_capacity(operation_size);
    let mut idx = 2;
    for _ in 0..operation_size {
        let op: i32 = tokens[idx].parse().expect("Failed to parse operation as i32");
        operations.push(op);
        idx += 1;
    }

    // Compute and print the result character, followed by a newline.
    // Equivalent to: printf("%c\n", kthCharacter(k, operations, operationSize));
    let result_char = kth_character(k, &operations);
    println!("{}", result_char);

    Ok(())
}