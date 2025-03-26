use std::io::{self, Read};

/// This function calculates the number of substrings according to the logic of the original C++ code.
/// It iterates through the characters in the input string (assumed to be ASCII lowercase)
/// and uses a sliding window strategy with an array to count character frequencies.
fn number_of_substrings(s: &str, k: i32) -> i32 {
    // Convert the string to a vector of bytes.
    let s_bytes = s.as_bytes();
    let mut ans = 0;
    let mut left = 0;
    // Use an array of 26 integers initialized to 0 to count each letter ('a' to 'z').
    let mut cnt = [0_i32; 26];

    // Iterate over each byte in the string slice.
    for &c in s_bytes.iter() {
        // Compute the index for the current character assuming a is 0.
        let idx = (c - b'a') as usize;
        cnt[idx] += 1;
        // As long as the frequency of the current character is at least k,
        // move the left pointer to narrow the window and decrease the count accordingly.
        while cnt[idx] >= k {
            let left_idx = (s_bytes[left] - b'a') as usize;
            cnt[left_idx] -= 1;
            left += 1;
        }
        // Accumulate the number of valid substrings ending at the current position.
        ans += left as i32;
    }
    ans
}

fn main() -> io::Result<()> {
    // Read the entire input from standard input.
    // The input format is the same as in the original C++ code: 
    // a string s and an integer k provided (possibly on the same line or on multiple lines).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input into tokens using whitespace.
    let mut tokens = input.split_whitespace();

    // Parse the string s.
    let s = tokens
        .next()
        .expect("Expected a string input")
        .to_string();

    // Parse the integer k.
    let k = tokens
        .next()
        .expect("Expected an integer input for k")
        .parse::<i32>()
        .expect("Failed to parse k as an integer");

    // Compute the answer using the provided algorithm.
    let result = number_of_substrings(&s, k);

    // Print the result to standard output. The output format exactly matches the original C++ code.
    println!("{}", result);

    Ok(())
}