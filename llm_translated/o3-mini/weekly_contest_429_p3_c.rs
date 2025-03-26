use std::cmp;
use std::io::{self, Read};

// Constant for comparison to avoid processing very large cost values
const COST_LIMIT: i32 = 100_000_000;

// This function updates the temporary DP array (temp_dp) based on the state in dp.
// We consider two possibilities: keeping the bit the same or flipping it.
fn update_cost_array(
    dp: &[[i32; 2]],
    temp_dp: &mut [[i32; 2]],
    max_len: usize,
    bin_str: &[u8],
    idx: usize,
    len: usize,
    bit_val: usize,
) {
    let current_cost = dp[len][bit_val];
    // If the cost is already too high, skip further processing.
    if current_cost > COST_LIMIT {
        return;
    }

    // Determine the value of the current character in the binary string.
    // (bin_str[idx] - b'0') converts the ASCII digit to its integer value.
    let current_digit = (bin_str[idx] - b'0') as i32;

    // Cost for keeping the bit unchanged.
    let cost_keep = current_cost + if current_digit != bit_val as i32 { 1 } else { 0 };
    // If extending the current substring is allowed (does not exceed max_len),
    // update the temporary DP array for the case of keeping the bit.
    if len < max_len {
        temp_dp[len + 1][bit_val] = cmp::min(temp_dp[len + 1][bit_val], cost_keep);
    }

    // Cost for flipping the bit.
    let flipped_bit = 1 - bit_val;
    let cost_flip = current_cost + if current_digit != (flipped_bit as i32) { 1 } else { 0 };
    // Always "restart" a new substring (with length 1) when flipping.
    temp_dp[1][flipped_bit] = cmp::min(temp_dp[1][flipped_bit], cost_flip);
}

// This function checks if it is possible to achieve a valid transformation with at most max_flips
// using a substring length of max_substr_len.
// It uses a dynamic programming approach to keep track of the minimum number of flips needed.
fn can_achieve(bin_str: &[u8], str_len: usize, max_substr_len: usize, max_flips: i32) -> bool {
    // Initialize the DP arrays. We only use indices 1..=max_substr_len.
    let mut dp = vec![[i32::MAX; 2]; max_substr_len + 1];
    let mut temp_dp = vec![[i32::MAX; 2]; max_substr_len + 1];

    // Set the starting condition using the first character of the binary string.
    let first_digit = (bin_str[0] - b'0') as usize;
    dp[1][first_digit] = 0;
    dp[1][1 - first_digit] = 1;

    // Process each subsequent character in the binary string.
    for idx in 1..str_len {
        // For every current substring length and for both possible bit values, try updating the DP state.
        for len in 1..=max_substr_len {
            for bit_val in 0..2 {
                update_cost_array(&dp, &mut temp_dp, max_substr_len, bin_str, idx, len, bit_val);
            }
        }

        // Copy the temporary DP array back into the main DP array for the next iteration,
        // and reset the temporary array back to i32::MAX.
        for len in 1..=max_substr_len {
            dp[len][0] = temp_dp[len][0];
            dp[len][1] = temp_dp[len][1];
            temp_dp[len][0] = i32::MAX;
            temp_dp[len][1] = i32::MAX;
        }
    }

    // Find the minimum number of flips required among all valid substring lengths.
    let mut min_flips = i32::MAX;
    for len in 1..=max_substr_len {
        min_flips = cmp::min(min_flips, cmp::min(dp[len][0], dp[len][1]));
    }

    min_flips <= max_flips
}

// This function performs binary search on the possible substring lengths (from 1 to str_len)
// to find the minimum substring length that can achieve the desired transformation with at most max_flips.
fn min_length(bin_str: &[u8], max_flips: i32) -> usize {
    let str_len = bin_str.len();
    let mut left: usize = 1;
    let mut right: usize = str_len;

    while left < right {
        let mid = (left + right) / 2;
        if can_achieve(bin_str, str_len, mid, max_flips) {
            // If it's possible to achieve with 'mid', try a smaller length.
            right = mid;
        } else {
            // Otherwise, try a larger substring length.
            left = mid + 1;
        }
    }

    left
}

// Main function to handle input and output.
// Input Format:
//   The first token is a binary string.
//   The second token is an integer representing the maximum number of allowed flips.
// Output Format:
//   A single integer representing the minimum substring length required.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Tokenize the input by whitespace.
    let mut tokens = input.split_whitespace();

    // Parse the binary string.
    let bin_str = tokens.next().ok_or("Missing binary string")?;
    // Parse the maximum flips allowed.
    let max_flips: i32 = tokens
        .next()
        .ok_or("Missing max flips")?
        .parse()
        .map_err(|_| "Invalid integer for max flips")?;

    // Convert the binary string to bytes for efficient processing.
    let bin_str_bytes = bin_str.as_bytes();

    // Calculate the minimum substring length.
    let result = min_length(bin_str_bytes, max_flips);

    // Output the result in the same format as the original C program.
    println!("{}", result);

    Ok(())
}