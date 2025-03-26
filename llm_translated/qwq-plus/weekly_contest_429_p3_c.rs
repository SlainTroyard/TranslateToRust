use std::io;

fn can_achieve(bin_str: &str, str_len: usize, max_substring_len: usize, max_flips: i32) -> bool {
    let mut dp = vec![[std::i32::MAX; 2]; max_substring_len + 1];
    let mut temp_dp = vec![[std::i32::MAX; 2]; max_substring_len + 1];

    // Initialize DP arrays
    for len in 0..=max_substring_len {
        for bit_val in 0..2 {
            dp[len][bit_val] = std::i32::MAX;
            temp_dp[len][bit_val] = std::i32::MAX;
        }
    }

    // Initialize first character
    let first_char = bin_str.chars().nth(0).unwrap() as u8 - b'0';
    dp[1][first_char as usize] = 0;
    dp[1][1 - first_char as usize] = 1;

    for idx in 1..str_len {
        // Process all len and bit_val
        for len in 1..=max_substring_len {
            for bit_val in 0..2 {
                let bit_val = bit_val as i32;
                update_cost_array(
                    &dp,
                    &mut temp_dp,
                    max_substring_len,
                    bin_str,
                    idx,
                    len,
                    bit_val,
                );
            }
        }

        // Copy temp_dp to dp and reset temp_dp
        for len in 1..=max_substring_len {
            for bit_val in 0..2 {
                dp[len][bit_val] = temp_dp[len][bit_val];
                temp_dp[len][bit_val] = std::i32::MAX;
            }
        }
    }

    // Find the minimum flips
    let mut min_flips = std::i32::MAX;
    for len in 1..=max_substring_len {
        for bit_val in 0..2 {
            if dp[len][bit_val] < min_flips {
                min_flips = dp[len][bit_val];
            }
        }
    }

    min_flips <= max_flips
}

fn update_cost_array(
    dp: &[[i32; 2]],
    temp_dp: &mut [[i32; 2]],
    max_substring_len: usize,
    bin_str: &str,
    idx: usize,
    len: usize,
    bit_val: i32,
) {
    let current_cost = dp[len][bit_val as usize];
    if current_cost > 100_000_000 {
        return;
    }

    // Get current bit
    let current_bit = bin_str.chars().nth(idx).unwrap() as u8 - b'0';
    let cost_keep = current_cost + if (current_bit != bit_val as u8) { 1 } else { 0 };

    if len < max_substring_len {
        let new_len = len + 1;
        if cost_keep < temp_dp[new_len][bit_val as usize] {
            temp_dp[new_len][bit_val as usize] = cost_keep;
        }
    }

    // Cost to flip the bit
    let opposite_bit_val = 1 - bit_val;
    let cost_flip = current_cost + if (current_bit != opposite_bit_val as u8) { 1 } else { 0 };

    if cost_flip < temp_dp[1][opposite_bit_val as usize] {
        temp_dp[1][opposite_bit_val as usize] = cost_flip;
    }
}

fn min_length(bin_str: &str, max_flips: i32) -> i32 {
    let str_len = bin_str.len();
    let mut left = 1;
    let mut right = str_len as i32;

    while left < right {
        let mid = (left + right) / 2;
        if can_achieve(bin_str, str_len, mid as usize, max_flips) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        panic!("Input must have exactly two parts");
    }
    let bin_str = parts[0];
    let max_flips: i32 = parts[1].parse().expect("Second part must be integer");

    let result = min_length(bin_str, max_flips);
    println!("{}", result);
}