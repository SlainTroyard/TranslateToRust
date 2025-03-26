use std::io;

const MAX_LEN: usize = 1000;
const INT_MAX: i32 = i32::MAX;

fn initialize_dp_arrays(max_len: usize, dp: &mut [[i32; 2]; MAX_LEN + 1], temp_dp: &mut [[i32; 2]; MAX_LEN + 1]) {
    for i in 0..=max_len {
        for j in 0..2 {
            dp[i][j] = INT_MAX;
            temp_dp[i][j] = INT_MAX;
        }
    }
}

fn update_cost_array(
    dp: &[[i32; 2]; MAX_LEN + 1],
    temp_dp: &mut [[i32; 2]; MAX_LEN + 1],
    max_len: usize,
    bin_str: &str,
    idx: usize,
    len: usize,
    bit_val: usize,
) {
    let current_cost = dp[len][bit_val];
    if current_cost > 1e8 as i32 {
        return;
    }

    // Cost to keep the bit the same
    let cost_keep = current_cost + ((bin_str.as_bytes()[idx] - b'0') != bit_val as u8) as i32;
    if len < max_len {
        temp_dp[len + 1][bit_val] = temp_dp[len + 1][bit_val].min(cost_keep);
    }

    // Cost to flip the bit
    let cost_flip = current_cost + ((bin_str.as_bytes()[idx] - b'0') != (1 - bit_val) as u8) as i32;
    temp_dp[1][1 - bit_val] = temp_dp[1][1 - bit_val].min(cost_flip);
}

fn can_achieve(bin_str: &str, str_len: usize, max_substr_len: usize, max_flips: i32) -> bool {
    let mut dp = [[INT_MAX; 2]; MAX_LEN + 1];
    let mut temp_dp = [[INT_MAX; 2]; MAX_LEN + 1];

    initialize_dp_arrays(max_substr_len, &mut dp, &mut temp_dp);

    dp[1][(bin_str.as_bytes()[0] - b'0') as usize] = 0;
    dp[1][1 - (bin_str.as_bytes()[0] - b'0') as usize] = 1;

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
                temp_dp[len][bit_val] = INT_MAX;
            }
        }
    }

    // Find the minimum flips required for any valid substring length
    let min_flips = (1..=max_substr_len)
        .flat_map(|len| [dp[len][0], dp[len][1]])
        .min()
        .unwrap_or(INT_MAX);

    min_flips <= max_flips
}

fn min_length(bin_str: &str, max_flips: i32) -> i32 {
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

    left as i32
}

fn main() {
    let mut bin_str = String::new();
    let mut max_flips = String::new();

    // Input the binary string
    io::stdin().read_line(&mut bin_str).expect("Failed to read line");
    bin_str = bin_str.trim().to_string();

    // Input the maximum number of flips allowed
    io::stdin().read_line(&mut max_flips).expect("Failed to read line");
    let max_flips: i32 = max_flips.trim().parse().expect("Please type a number!");

    // Calculate the minimum substring length
    let result = min_length(&bin_str, max_flips);

    // Output the result
    println!("{}", result);
}