use std::io::{self, BufRead};
use std::cmp::{min, max};

/// Function that implements the problem logic
/// It takes a string `s` representing the path and an integer `k`
/// Returns the maximum distance according to the given algorithm.
fn max_distance(s: &str, k: i32) -> i32 {
    let mut ans = 0;
    let (mut x, mut y) = (0, 0);
    // Enumerate over characters in the string
    for (i, ch) in s.chars().enumerate() {
        match ch {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            _   => x -= 1,  // For 'W'
        }
        // Calculate Manhattan distance with potential extra moves from k extra steps (each extra step can contribute 2 to the distance)
        let current_distance = (x.abs() + y.abs() + k * 2) as i32;
        // The actual distance cannot exceed the number of moves taken (i+1)
        ans = max(ans, min(current_distance, (i + 1) as i32));
    }
    ans
}

fn main() -> io::Result<()> {
    // Create an input buffer to read from standard input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first non-empty line for the input tokens. Since the original code uses "cin >> s >> k",
    // we assume that s and k may be on the same line or separated by whitespace.
    // We accumulate tokens until we have at least two.
    let mut tokens = Vec::new();
    while tokens.len() < 2 {
        if let Some(line) = lines.next() {
            let line = line?;
            // Split the current line into tokens (whitespace delimited)
            tokens.extend(line.split_whitespace().map(String::from));
        } else {
            break; // End of input
        }
    }

    // Ensure we have exactly two tokens: first the string s, then the integer k.
    if tokens.len() < 2 {
        eprintln!("Insufficient input provided.");
        return Ok(());
    }
    
    let s = tokens[0].clone();
    let k: i32 = tokens[1].parse().expect("Failed to parse k as an integer");

    // Call the solution function and print the result
    println!("{}", max_distance(&s, k));

    Ok(())
}