use std::cmp::{min, max};
use std::io::{self, Write};

const MAX_LEN: usize = 1000;
const INF: i32 = i32::MAX;

fn initialize_dp_arrays(max_len: usize) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut dp = vec![vec![INF; 2]; max_len + 1];
    let mut temp_dp = vec![vec![INF; 2]; max_len + 1];
    (dp, temp_dp)
}

fn update_cost_array(
    dp: &Vec<Vec<i32>>,
    temp_dp: &mut Vec<Vec<i32>>,
    max_len: usize,
    bin_str: &[u8],
    idx: usize,
    len: usize,
    bit_val: usize,
) {
    let current_cost = dp[len][bit_val];
    // Skip if the cost exceeds a large threshold
    if current_cost > 1e8 as i32 {
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
    let (mut dp, mut temp_dp) = initialize_dp_arrays(max_substr_len);

    dp[1][(bin_str[0] - b'0') as usize] = 0;
    dp[1][1 - (bin_str[0] - b'0') as usize] = 1;

    for idx in 1..str_len {
        for len in 1..=max_substr_len {
            for bit_val in 0..2 {
                update_cost_array(&dp, &mut temp_dp, max_substr_len, bin_str, idx, len, bit_val);
            }
        }

        // Copy temp_dp to dp for the next iteration
        for len in 1..=max_substr_len {
            for bit_val in 0..2 {
                dp[len][bit_val] = temp_dp[len][bit_val];
                temp_dp[len][bit_val] = INF;
            }
        }
    }

    // Find the minimum flips required for any valid substring length
    dp.iter()
        .flat_map(|sub| sub.iter())
        .cloned()
        .min()
        .unwrap_or(INF)
        <= max_flips
}

fn min_length(bin_str: &str, max_flips: i32) -> usize {
    let str_len = bin_str.len();
    let bin_str = bin_str.as_bytes();
    let mut left = 1;
    let mut right = str_len;

    while left < right {
        let mid = (left + right) / 2;
        if can_achieve(bin_str, str_len, mid, max_flips) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn main() {
    // Input handling
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read bin_str");
    let bin_str = input.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read max_flips");
    let max_flips: i32 = input.trim().parse().expect("max_flips must be an integer");

    // Calculate the minimum substring length
    let result = min_length(&bin_str, max_flips);

    // Print the result
    println!("{}", result);
}