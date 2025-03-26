use std::io::{self, BufRead};

/// Recursively searches for the k-th character in the string
/// 
/// This function determines the character at position k in the string
/// after applying operations recursively through the binary tree representation.
fn kchar_search(k: i64, operations: &[i32], pos: usize) -> char {
    let mut pow_sum: i64 = 1;
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
        let mut next_char = ((kchar as u8) + 1) as char;
        if next_char > 'z' {
            next_char = 'a';
        }
        return next_char;
    }

    kchar_search(k - pow_sum / 2, operations, tmp_pos - 1)
}

/// Finds the k-th character in the string after applying operations
fn kth_character(k: i64, operations: &[i32], _operations_size: usize) -> char {
    let mut pow_sum: i64 = 1;
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line containing k and operationSize
    let first_line = lines.next().unwrap()?;
    let mut values = first_line.trim().split_whitespace();
    
    let k: i64 = values.next().unwrap().parse().expect("Failed to parse k");
    let operation_size: usize = values.next().unwrap().parse().expect("Failed to parse operationSize");
    
    // Read the second line containing operations
    let second_line = lines.next().unwrap()?;
    let operations: Vec<i32> = second_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse operation"))
        .collect();
    
    // Call the function and print the result
    println!("{}", kth_character(k, &operations, operation_size));
    
    Ok(())
}