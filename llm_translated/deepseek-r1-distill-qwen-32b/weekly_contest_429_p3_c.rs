use std::io;
use std::cmp;

const MAX_LEN: usize = 1000;

fn initialize_dp(max_len: usize) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut dp = vec![vec![i32::MAX; 2]; max_len + 1];
    let mut temp_dp = vec![vec![i32::MAX; 2]; max_len + 1];
    
    for i in 0..=max_len {
        for j in 0..2 {
            dp[i][j] = i32::MAX;
            temp_dp[i][j] = i32::MAX;
        }
    }
    
    (dp, temp_dp)
}

fn update_cost(dp: &Vec<Vec<i32>>, temp_dp: &mut Vec<Vec<i32>>, max_len: usize, bin_str: &str, idx: usize, len: usize, bit_val: usize) {
    let current_cost = dp[len][bit_val];
    if current_cost > 100000000 {
        return;
    }
    
    // Cost to keep the bit the same
    let cost_keep = current_cost + if bin_str.chars().nth(idx).unwrap() != (bit_val as u8 + b'0') as char { 1 } else { 0 };
    if len < max_len {
        temp_dp[len + 1][bit_val] = cmp::min(temp_dp[len + 1][bit_val], cost_keep);
    }
    
    // Cost to flip the bit
    let cost_flip = current_cost + if bin_str.chars().nth(idx).unwrap() != ((1 - bit_val) as u8 + b'0') as char { 1 } else { 0 };
    temp_dp[1][1 - bit_val] = cmp::min(temp_dp[1][1 - bit_val], cost_flip);
}

fn can_achieve(bin_str: &str, str_len: usize, max_sub_len: usize, max_flips: i32) -> bool {
    let (mut dp, mut temp_dp) = initialize_dp(max_sub_len);
    
    let first_char = bin_str.chars().nth(0).unwrap();
    let first_bit = (first_char as u8 - b'0') as usize;
    dp[1][first_bit] = 0;
    dp[1][1 - first_bit] = 1;
    
    for idx in 1..str_len {
        for len in 1..=max_sub_len {
            for bit_val in 0..2 {
                update_cost(&dp, &mut temp_dp, max_sub_len, bin_str, idx, len, bit_val);
            }
        }
        
        // Copy temp_dp to dp and reset temp_dp
        for len in 1..=max_sub_len {
            for bit_val in 0..2 {
                dp[len][bit_val] = temp_dp[len][bit_val];
                temp_dp[len][bit_val] = i32::MAX;
            }
        }
    }
    
    // Find minimum flips
    let mut min_flips = i32::MAX;
    for len in 1..=max_sub_len {
        for bit_val in 0..2 {
            if dp[len][bit_val] < min_flips {
                min_flips = dp[len][bit_val];
            }
        }
    }
    
    min_flips <= max_flips
}

fn min_length(bin_str: &str, max_flips: i32) -> usize {
    let str_len = bin_str.len();
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
    let mut bin_str = String::new();
    io::stdin().read_line(&mut bin_str).expect("Failed to read binary string");
    bin_str = bin_str.trim().to_string();
    
    let mut max_flips_str = String::new();
    io::stdin().read_line(&mut max_flips_str).expect("Failed to read max flips");
    let max_flips = max_flips_str.trim().parse::<i32>().expect("Invalid max flips");
    
    let result = min_length(&bin_str, max_flips);
    println!("{}", result);
}