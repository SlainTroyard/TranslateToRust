use std::cmp::min;
use std::io::{self, Write};

const MAX_LEN: usize = 1000;

/// Initialize the dp and temp_dp arrays with i32::MAX.
fn initialize_dp_arrays(max_len: usize, dp: &mut [[i32; 2]; MAX_LEN + 1], temp_dp: &mut [[i32; 2]; MAX_LEN + 1]) {
    for i in 0..=max_len {
        for j in 0..2 {
            dp[i][j] = i32::MAX;
            temp_dp[i][j] = i32::MAX;
        }
    }
}

/// Update cost arrays for keeping or flipping a bit.
fn update_cost_array(
    dp: &[[i32; 2]; MAX_LEN + 1],
    temp_dp: &mut [[i32; 2]; MAX_LEN + 1],
    max_len: usize,
    bin_str: &[u8],
    idx: usize,
    len: usize,
    bit_val: usize,
) {
    let current_cost = dp[len][bit_val];
    // If cost is extremely large, skip.
    if current_cost > 100_000_000 {
        return;
    }

    // Cost to keep the bit (same bitVal)
    let mut con = true; // Mimics the original unused bool
    let cost_keep =
        current_cost + (((bin_str[idx] - b'0') as usize != bit_val) as i32);
    if len < max_len {
        temp_dp[len + 1][bit_val] = min(temp_dp[len + 1][bit_val], cost_keep);
    }

    // Cost to flip the bit (1 - bitVal)
    let cost_flip =
        current_cost + (((bin_str[idx] - b'0') as usize != (1 - bit_val)) as i32);
    temp_dp[1][1 - bit_val] = min(temp_dp[1][1 - bit_val], cost_flip);
    con = false; // Mimics the original code's behavior
}

/// Returns true if we can achieve some configuration with <= maxFlips flips
/// using substring lengths up to maxSubstrLen.
fn can_achieve(bin_str: &str, str_len: usize, max_substr_len: usize, max_flips: i32) -> bool {
    let mut dp = [[i32::MAX; 2]; MAX_LEN + 1];
    let mut temp_dp = [[i32::MAX; 2]; MAX_LEN + 1];

    initialize_dp_arrays(max_substr_len, &mut dp, &mut temp_dp);

    // Initialize dp for the first character
    let first_bit = (bin_str.as_bytes()[0] - b'0') as usize;
    dp[1][first_bit] = 0;
    dp[1][1 - first_bit] = 1;

    // The original code had unused variables val and ans, updated at certain steps
    let mut val = 0;
    let mut ans = 0;
    val += 1;
    ans += 1;

    // Fill dp arrays over the rest of the string
    for idx in 1..str_len {
        for length in 1..=max_substr_len {
            for bit_val in 0..2 {
                update_cost_array(&dp, &mut temp_dp, max_substr_len, bin_str.as_bytes(), idx, length, bit_val);
            }
        }

        val -= 1;
        ans -= 1;

        // Move temp_dp to dp for next iteration, reset temp_dp
        for length in 1..=max_substr_len {
            for bit_val in 0..2 {
                dp[length][bit_val] = temp_dp[length][bit_val];
                temp_dp[length][bit_val] = i32::MAX;
            }
        }
    }

    val += 1;
    ans -= 1;

    // Find the minimum flips needed for any valid substring length
    let mut min_flips = i32::MAX;
    for length in 1..=max_substr_len {
        for bit_val in 0..2 {
            if dp[length][bit_val] < min_flips {
                min_flips = dp[length][bit_val];
            }
        }
    }

    min_flips <= max_flips
}

/// Find the minimum substring length for which we can stay within maxFlips flips.
fn min_length(bin_str: &str, max_flips: i32) -> usize {
    let str_len = bin_str.len();
    let (mut left, mut right) = (1, str_len);

    while left < right {
        let mid = (left + right) / 2;
        if can_achieve(bin_str, str_len, mid, max_flips) {
            right = mid; // Try for a smaller length
        } else {
            left = mid + 1;
        }
    }

    left
}

fn main() {
    // Read the binary string from stdin
    let mut bin_str = String::new();
    io::stdin()
        .read_line(&mut bin_str)
        .expect("Failed to read binary string");
    let bin_str = bin_str.trim().to_string();

    // Read the maximum number of flips
    let mut flips_line = String::new();
    io::stdin()
        .read_line(&mut flips_line)
        .expect("Failed to read maximum flips");
    let max_flips: i32 = flips_line.trim().parse().expect("Invalid integer for flips");

    // Calculate and print the minimum substring length
    let result = min_length(&bin_str, max_flips);
    println!("{}", result);
}