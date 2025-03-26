use std::cmp::min;
use std::io::{self, Write};

const MAX_LEN: usize = 1000;

fn initialize_dp_arrays(max_len: usize, dp: &mut [[i32; 2]], temp_dp: &mut [[i32; 2]]) {
    for i in 0..=max_len {
        for j in 0..2 {
            dp[i][j] = i32::MAX;
            temp_dp[i][j] = i32::MAX;
        }
    }
}

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
    if current_cost > 1_000_000_000 {
        return;
    }

    // Cost to keep the bit the same
    let cost_keep = current_cost + ((bin_str[idx] - b'0') as usize != bit_val) as i32;
    if len < max_len {
        temp_dp[len + 1][bit_val] = min(temp_dp[len + 1][bit_val], cost_keep);
    }

    // Cost to flip the bit
    let cost_flip = current_cost + ((bin_str[idx] - b'0') as usize != (1 - bit_val)) as i32;
    temp_dp[1][1 - bit_val] = min(temp_dp[1][1 - bit_val], cost_flip);
}

fn can_achieve(bin_str: &[u8], str_len: usize, max_substr_len: usize, max_flips: i32) -> bool {
    let mut dp = [[i32::MAX; 2]; MAX_LEN + 1];
    let mut temp_dp = [[i32::MAX; 2]; MAX_LEN + 1];

    initialize_dp_arrays(max_substr_len, &mut dp, &mut temp_dp);

    dp[1][(bin_str[0] - b'0') as usize] = 0;
    dp[1][1 - (bin_str[0] - b'0') as usize] = 1;

    for idx in 1..str_len {
        for len in 1..=max_substr_len {
            for bit_val in 0..2 {
                update_cost_array(&dp, &mut temp_dp, max_substr_len, bin_str, idx, len, bit_val);
            }
        }

        // Copy the temporary DP array to the main DP array for the next iteration
        for len in 1..=max_substr_len {
            for bit_val in 0..2 {
                dp[len][bit_val] = temp_dp[len][bit_val];
                temp_dp[len][bit_val] = i32::MAX;
            }
        }
    }

    // Find the minimum flips required for any valid substring length
    let mut min_flips = i32::MAX;
    for len in 1..=max_substr_len {
        for bit_val in 0..2 {
            min_flips = min(min_flips, dp[len][bit_val]);
        }
    }

    min_flips <= max_flips
}

fn min_length(bin_str: &[u8], max_flips: i32) -> usize {
    let str_len = bin_str.len();
    let mut left = 1;
    let mut right = str_len;

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
    let mut bin_str = String::new();
    io::stdin().read_line(&mut bin_str).expect("Failed to read line");
    let bin_str = bin_str.trim().as_bytes();

    let mut max_flips_str = String::new();
    io::stdin().read_line(&mut max_flips_str).expect("Failed to read line");
    let max_flips: i32 = max_flips_str.trim().parse().expect("Failed to parse max_flips");

    let result = min_length(bin_str, max_flips);
    println!("{}", result);
}