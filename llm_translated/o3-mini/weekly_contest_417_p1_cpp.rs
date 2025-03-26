use std::io::{self, BufRead};

/// Computes the kth character according to the algorithm:
/// kth character = 'a' + popcount(k - 1)
/// where popcount(k - 1) counts the number of 1’s in the binary representation of (k - 1)
fn kth_character(k: u64) -> char {
    // count_ones returns a u32 so we cast it to u8 before adding to b'a'
    let popcount = (k - 1).count_ones() as u8;
    // b'a' is the ASCII code for 'a'. Add popcount and convert to char.
    (b'a' + popcount) as char
}

fn main() -> io::Result<()> {
    // Read the entire standard input into a String
    // This supports multiple lines or multiple tokens per line, just like the original C++ code.
    let stdin = io::stdin();
    let input = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?.join(" ");

    // Split the collected input on whitespace and take the first token as the integer k.
    let k_str = input.split_whitespace().next().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "No input provided")
    })?;

    // Parse the token into a u64. We use u64 as it corresponds to long long in C++.
    let k: u64 = k_str.trim().parse().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("Failed to parse k: {}", e))
    })?;

    // Compute the kth character using our function. Mimics `Solution::kthCharacter` in C++.
    let result_char = kth_character(k);

    // Output the result followed by a newline, exactly matching C++ code's output.
    println!("{}", result_char);

    Ok(())
}