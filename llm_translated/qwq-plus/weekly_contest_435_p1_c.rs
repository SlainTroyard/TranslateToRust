use std::io;

/// Computes the maximum difference between the largest odd frequency and the smallest even non-zero frequency of characters in a string.
///
/// # Arguments
///
/// * `s` - A string slice containing only lowercase letters.
///
/// # Returns
///
/// The difference between the largest odd frequency and the smallest even frequency.
fn max_difference(s: &str) -> i32 {
    let mut cnt = [0i32; 26];
    // Count frequency of each character
    for &c in s.as_bytes() {
        let idx = (c - b'a') as usize;
        cnt[idx] += 1;
    }
    
    let (mut c1, mut c2) = (0, 100); // Initialize c1 to 0 (min possible) and c2 to 100 (arbitrary high value)
    // Find max odd count and min even non-zero count
    for &count in &cnt {
        if count % 2 == 1 {
            if count > c1 {
                c1 = count;
            }
        } else if count != 0 {
            if count < c2 {
                c2 = count;
            }
        }
    }
    
    c1 - c2
}

fn main() {
    let mut input = String::new();
    // Read input line (equivalent to scanf("%s" in C, but safely handles buffer)
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim(); // Remove any leading/trailing whitespace (including newline)
    // Compute and print result with newline (matches original C's printf)
    println!("{}", max_difference(input));
}