use std::io::{self, Read, Write};

/// Recursively computes the kth character based on the operations array.
/// 
/// This function mimics the C function kchar_search.
/// 
/// Parameters:
/// - k: the k value (1-indexed)
/// - operations: slice of operations values (each treated as boolean: 0=false, nonzero=true)
/// - pos: current index in the operations array that controls whether to "transform" the result
///
/// Returns: the computed character as per the algorithm.
fn kchar_search(k: i64, operations: &[i32], pos: usize) -> char {
    // Base case: if pos is 0 (i.e., we've reached the first operation)
    // or if k is 1, then return 'b' if the operation is active, else 'a'.
    if pos == 0 || k == 1 {
        return if operations[pos] != 0 { 'b' } else { 'a' };
    }

    // Compute the smallest power of two that is >= k.
    let mut pow_sum = 1;
    let mut tmp_pos = 0;
    while k > pow_sum {
        pow_sum *= 2;
        tmp_pos += 1;
    }

    // Determine the new k for the recursive call.
    let new_k = k - (pow_sum / 2);

    // If the current operation is active, transform the result.
    if operations[pos] != 0 {
        let mut kchar = kchar_search(new_k, operations, tmp_pos - 1);
        // Increment the character.
        // If incrementing goes past 'z', wrap back to 'a'.
        let next = (kchar as u8).wrapping_add(1);
        if next > b'z' {
            'a'
        } else {
            next as char
        }
    } else {
        // If the operation is not active, return the recursive result directly.
        kchar_search(new_k, operations, tmp_pos - 1)
    }
}

/// Computes the kth character given the operations array.
/// 
/// This function mimics the C function kthCharacter.
/// It figures out the proper starting index based on the initial k value.
fn kth_character(k: i64, operations: &[i32]) -> char {
    if k == 1 {
        return 'a';
    }
    let mut pow_sum = 1;
    let mut pos = 0;
    // Find the smallest power of two that is greater than or equal to k.
    while pow_sum < k {
        pow_sum *= 2;
        pos += 1;
    }
    // The initial call adjusts k and starts at index pos - 1.
    kchar_search(k - (pow_sum / 2), operations, pos - 1)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the entire standard input into a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input by whitespace to obtain tokens.
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Check that there are at least two tokens (k and operationSize).
    if tokens.len() < 2 {
        return Err("Not enough input values.".into());
    }

    // Parse the first two tokens: k and the number of operations.
    let k: i64 = tokens[0].parse()?;
    let operation_size: usize = tokens[1].parse()?;

    // After the first two tokens, there should be exactly `operation_size` numbers.
    if tokens.len() < 2 + operation_size {
        return Err("Not enough operation values.".into());
    }

    // Parse the operations into a vector of i32.
    let mut operations = Vec::with_capacity(operation_size);
    for i in 0..operation_size {
        let op: i32 = tokens[2 + i].parse()?;
        operations.push(op);
    }

    // Compute the kth character using the provided algorithm.
    let result = kth_character(k, &operations);

    // Write the result followed by a newline to standard output.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", result)?;

    Ok(())
}