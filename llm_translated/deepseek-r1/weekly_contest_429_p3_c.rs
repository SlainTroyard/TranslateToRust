use std::io::{self, BufRead};

const MAX_LEN: usize = 1000;

fn can_achieve(bin_str: &[u8], str_len: usize, max_substr_len: usize, max_flips: i32) -> bool {
    let mut dp = [[i32::MAX; 2]; MAX_LEN + 1];
    let mut temp_dp = [[i32::MAX; 2]; MAX_LEN + 1];

    // Initialize for the first character
    let first_bit = (bin_str[0] - b'0') as usize;
    dp[1][first_bit] = 0;
    dp[1][1 - first_bit] = 1;

    // Variables that are modified but not used (preserved from original code)
    let mut _val = 0;
    let mut _ans = 0;
    _val += 1;
    _ans += 1;

    for idx in 1..str_len {
        // Reset temp_dp for current index processing
        for len in 1..=max_substr_len {
            for bit_val in 0..2 {
                temp_dp[len][bit_val] = i32::MAX;
            }
        }

        // Update temp_dp based on current dp
        for len in 1..=max_substr_len {
            for bit_val in 0..2 {
                update_cost_array(&dp, &mut temp_dp, max_substr_len, bin_str, idx, len, bit_val);
            }
        }

        // Copy temp_dp back to dp and reset temp_dp
        for len in 1..=max_substr_len {
            for bit_val in 0..2 {
                dp[len][bit_val] = temp_dp[len][bit_val];
                temp_dp[len][bit_val] = i32::MAX;
            }
        }

        _val -= 1;
        _ans -= 1;
    }

    _val += 1;
    _ans -= 1;

    // Find the minimum flips required
    let mut min_flips = i32::MAX;
    for len in 1..=max_substr_len {
        for bit_val in 0..2 {
            min_flips = min_flips.min(dp[len][bit_val]);
        }
    }

    min_flips <= max_flips
}

fn update_cost_array(
    dp: &[[i32; 2]; MAX_LEN + 1],
    temp_dp: &mut [[i32; 2]; MAX_LEN + 1],
    max_substr_len: usize,
    bin_str: &[u8],
    idx: usize,
    len: usize,
    bit_val: usize,
) {
    let current_cost = dp[len][bit_val];
    if current_cost > 1_000_000_000 {
        return;
    }

    let current_char = bin_str[idx];
    let expected_bit = bit_val as u8;

    // Cost to keep the bit
    let cost_keep = current_cost + if (current_char - b'0') != expected_bit { 1 } else { 0 };
    if len < max_substr_len {
        let new_len = len + 1;
        if cost_keep < temp_dp[new_len][bit_val] {
            temp_dp[new_len][bit_val] = cost_keep;
        }
    }

    // Cost to flip the bit
    let flipped_bit = 1 - bit_val;
    let cost_flip = current_cost + if (current_char - b'0') != flipped_bit as u8 { 1 } else { 0 };
    if cost_flip < temp_dp[1][flipped_bit] {
        temp_dp[1][flipped_bit] = cost_flip;
    }
}

fn min_length(bin_str: &str, max_flips: i32) -> i32 {
    let str_len = bin_str.len();
    if str_len == 0 {
        return 0;
    }

    let bin_bytes = bin_str.as_bytes();
    let mut left = 1;
    let mut right = str_len;

    while left < right {
        let mid = (left + right) / 2;
        if can_achieve(bin_bytes, str_len, mid, max_flips) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left as i32
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read binary string
    let bin_str = lines.next().unwrap().expect("Failed to read binary string");
    let bin_str = bin_str.trim();

    // Read max flips
    let max_flips: i32 = lines.next().unwrap().unwrap().trim().parse().expect("Invalid max flips");

    let result = min_length(bin_str, max_flips);
    println!("{}", result);
}