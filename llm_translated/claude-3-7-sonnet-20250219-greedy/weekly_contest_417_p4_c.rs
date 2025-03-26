use std::io::{self, BufRead};

/// Recursively searches for the kth character in the sequence
fn kchar_search(k: i64, operations: &[i32], pos: usize) -> char {
    let mut pow_sum: i64 = 1;
    let mut tmp_pos = 0;
    
    // Base case: if position is 0 or k is 1
    if pos == 0 || k == 1 {
        if operations[pos] == 1 {
            return 'b';
        }
        return 'a';
    }

    // Find the position in the sequence
    while k > pow_sum {
        pow_sum *= 2;
        tmp_pos += 1;
    }

    if operations[pos] == 1 {
        // If operation is 1, increment the character
        let kchar = kchar_search(k - pow_sum / 2, operations, tmp_pos - 1);
        let mut next_char = kchar as u8 + 1;
        if next_char > b'z' {
            return 'a';
        }
        return next_char as char;
    }

    // If operation is 0, just continue the search
    kchar_search(k - pow_sum / 2, operations, tmp_pos - 1)
}

/// Finds the kth character in the sequence after applying operations
fn kth_character(k: i64, operations: &[i32], _operations_size: usize) -> char {
    let mut pow_sum: i64 = 1;
    let mut pos = 0;
    
    if k == 1 {
        return 'a';
    }

    // Find the position in the sequence
    while pow_sum < k {
        pow_sum *= 2;
        pos += 1;
    }
    
    kchar_search(k - pow_sum / 2, operations, pos - 1)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read k and operationSize
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let k: i64 = parts.next().unwrap().parse().unwrap();
    let operation_size: usize = parts.next().unwrap().parse().unwrap();
    
    // Read operations
    let second_line = lines.next().unwrap().unwrap();
    let operations: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and print the result
    println!("{}", kth_character(k, &operations, operation_size));
}