use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a buffered reader for better I/O performance.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input_line = String::new();

    // Read one line from stdin.
    reader.read_line(&mut input_line)?;
    // Trim the input and split by whitespace.
    let tokens: Vec<&str> = input_line.split_whitespace().collect();
    if tokens.len() < 2 {
        eprintln!("Error reading input");
        return Err("Invalid input: expected a string and an integer".into());
    }

    // The first token is the digit string, and the second token is k.
    let s = tokens[0].to_string();
    // We use usize for k since it is later used for length comparisons.
    let k: usize = tokens[1].parse().map_err(|_| "Invalid integer input")?;

    // Call the function to compute the answer.
    let result = max_difference(&s, k);

    // Print the result to stdout in the same format as the original C code.
    println!("{}", result);
    Ok(())
}

/// Computes the maximum difference as described in LeetCode Weekly Contest 435 Problem 4.
///
/// The algorithm iterates through all pairs (x, y) of digits (from 0 to 4) where x != y, and
/// for each pair, it uses a sliding window mechanism with two arrays (`cur_s` and `pre_s`) to
/// track digit counts. The array `min_s` is used to store the minimum difference values based on parity.
///
/// The logic closely follows the original C implementation.
fn max_difference(s: &str, k: usize) -> i32 {
    // Use half of the maximum i32 value to avoid potential overflows.
    let inf = std::i32::MAX / 2;
    let mut ans = -inf;
    let len = s.len();

    // Convert string digits into a vector of i32 values.
    // Assumes that the string contains only digit characters (i.e. '0' to '9').
    let s_bytes: Vec<i32> = s.bytes().map(|b| (b - b'0') as i32).collect();

    // Iterate through all pairs (x, y) where x and y are in [0, 4] and x != y.
    for x in 0..5 {
        for y in 0..5 {
            if x == y {
                continue;
            }

            // cur_s tracks the frequency of each digit in the current sliding window.
            let mut cur_s = [0; 5];
            // pre_s tracks the frequency of each digit before the current window.
            let mut pre_s = [0; 5];
            // min_s stores the minimal difference values based on the parity of pre_s[x] and pre_s[y].
            let mut min_s = [[inf, inf], [inf, inf]];
            // 'left' is the left boundary of the sliding window.
            let mut left = 0;

            // Iterate over the string characters using the index 'i'.
            for i in 0..len {
                // Update the count for the current digit in the window.
                let digit = s_bytes[i] as usize;
                cur_s[digit] += 1;
                let r = i + 1;

                // While the current window has at least k characters and the counts for x and y
                // are greater than in the left part, update min_s and slide the window.
                while r - left >= k && cur_s[x as usize] > pre_s[x as usize] && cur_s[y as usize] > pre_s[y as usize] {
                    // Determine the parity indices for pre_s[x] and pre_s[y].
                    let parity_x = (pre_s[x as usize] & 1) as usize;
                    let parity_y = (pre_s[y as usize] & 1) as usize;

                    // Update the minimal difference for this parity combination.
                    let diff = pre_s[x as usize] - pre_s[y as usize];
                    if diff < min_s[parity_x][parity_y] {
                        min_s[parity_x][parity_y] = diff;
                    }

                    // Move the left boundary: update pre_s with the digit being removed.
                    let left_digit = s_bytes[left] as usize;
                    pre_s[left_digit] += 1;
                    left += 1;
                }

                // Compute parity indices based on the current counts.
                let parity_cur_x = ((cur_s[x as usize] & 1) ^ 1) as usize;
                let parity_cur_y = (cur_s[y as usize] & 1) as usize;
                // Update the answer using the current counts and the stored minimum difference.
                let potential = cur_s[x as usize] - cur_s[y as usize] - min_s[parity_cur_x][parity_cur_y];
                if potential > ans {
                    ans = potential;
                }
            }
        }
    }
    ans
}