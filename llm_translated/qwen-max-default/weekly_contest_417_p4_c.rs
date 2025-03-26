use std::io::{self, BufRead, Write};

// Helper function to search for the kth character
fn kchar_search(k: usize, operations: &[i32], pos: usize) -> char {
    let mut pow_sum = 1;
    let mut tmp_pos = 0;

    if pos == 0 || k == 1 {
        if operations[pos] != 0 {
            return 'b';
        }
        return 'a';
    }

    while k > pow_sum {
        pow_sum *= 2;
        tmp_pos += 1;
    }

    if operations[pos] != 0 {
        let kchar = kchar_search(k - pow_sum / 2, operations, tmp_pos - 1);
        if kchar as u8 + 1 > b'z' {
            return 'a';
        }
        return (kchar as u8 + 1) as char;
    }

    kchar_search(k - pow_sum / 2, operations, tmp_pos - 1)
}

// Main function to find the kth character
fn kth_character(k: usize, operations: &[i32]) -> char {
    let mut pow_sum = 1;
    let mut pos = 0;

    if k == 1 {
        return 'a';
    }

    while pow_sum < k {
        pow_sum *= 2;
        pos += 1;
    }

    kchar_search(k - pow_sum / 2, operations, pos - 1)
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Read input
    let mut input = String::new();
    stdin.lock().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let k: usize = iter.next().unwrap().parse().expect("Invalid input for k");
    let operation_size: usize = iter.next().unwrap().parse().expect("Invalid input for operation size");

    // Allocate and read operations
    let mut operations = vec![0; operation_size];
    for i in 0..operation_size {
        input.clear();
        stdin.lock().read_line(&mut input).expect("Failed to read line");
        operations[i] = input.trim().parse().expect("Invalid input for operations");
    }

    // Find and print the kth character
    let result = kth_character(k, &operations);
    writeln!(stdout, "{}", result).expect("Failed to write output");
}