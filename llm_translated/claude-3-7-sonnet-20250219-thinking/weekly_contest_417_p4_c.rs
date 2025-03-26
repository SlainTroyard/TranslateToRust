use std::io::{self, Read};

/// Recursively search for the kth character based on the operations performed
fn kchar_search(k: i64, operations: &[i32], pos: usize) -> char {
    let mut pow_sum = 1;
    let mut tmp_pos = 0;
    
    // Base case: if position is 0 or k is 1
    if pos == 0 || k == 1 {
        if operations[pos] == 1 {
            return 'b';
        }
        return 'a';
    }

    // Find the position in the binary tree
    while k > pow_sum {
        pow_sum *= 2;
        tmp_pos += 1;
    }

    // If operation is 1, increment the character
    if operations[pos] == 1 {
        let kchar = kchar_search(k - pow_sum / 2, operations, tmp_pos - 1);
        let next_char = ((kchar as u8) + 1) as char;
        if next_char > 'z' {
            return 'a';
        }
        return next_char;
    }

    // Otherwise, pass through the character
    kchar_search(k - pow_sum / 2, operations, tmp_pos - 1)
}

/// Find the kth character after applying all operations
fn kth_character(k: i64, operations: &[i32], _operations_size: usize) -> char {
    let mut pow_sum = 1;
    let mut pos = 0;
    
    // Base case: first character is always 'a'
    if k == 1 {
        return 'a';
    }

    // Find the position in the binary tree
    while pow_sum < k {
        pow_sum *= 2;
        pos += 1;
    }
    
    kchar_search(k - pow_sum / 2, operations, pos - 1)
}

fn main() {
    // Read all input into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut words = input.split_whitespace();
    
    // Parse k and operationsSize
    let k = words.next().expect("Missing k")
                 .parse::<i64>().expect("Invalid k value");
    let operations_size = words.next().expect("Missing operations size")
                               .parse::<usize>().expect("Invalid operations size");
    
    // Parse operations array
    let mut operations = Vec::with_capacity(operations_size);
    for _ in 0..operations_size {
        let op = words.next().expect("Missing operation value")
                     .parse::<i32>().expect("Invalid operation value");
        operations.push(op);
    }
    
    // Calculate and print the result
    let result = kth_character(k, &operations, operations_size);
    println!("{}", result);
}