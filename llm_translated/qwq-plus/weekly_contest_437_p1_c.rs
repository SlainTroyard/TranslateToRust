use std::io;

/// Checks if the string contains a substring of exactly k identical characters.
///
/// # Arguments
///
/// * `s` - The input string to check.
/// * `k` - The required length of the consecutive characters.
///
/// # Returns
///
/// * `true` if a special substring exists, `false` otherwise.
pub fn has_special_substring(s: &str, k: i32) -> bool {
    if s.is_empty() || k < 1 {
        return false;
    }
    let target = k as usize;
    let bytes = s.as_bytes();
    let mut prev = bytes[0];
    let mut run_length = 1;
    for &current in &bytes[1..] {
        if current == prev {
            run_length += 1;
        } else {
            if run_length == target {
                return true;
            }
            prev = current;
            run_length = 1;
        }
    }
    run_length == target
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
    if parts.len() != 2 {
        panic!("Input must contain exactly two parts: a string and an integer");
    }
    
    let s = parts[0];
    let k: i32 = parts[1]
        .trim()
        .parse()
        .expect("Second part must be a valid integer");
    
    let result = has_special_substring(s, k);
    println!("{}", if result { 1 } else { 0 });
}