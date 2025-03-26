use std::io::{self, Read};

fn count_winning_sequences(s: &str) -> i32 {
    const MAX_SIZE: i32 = 1_000_000_007;

    if s.is_empty() {
        return 0;
    }

    // Initialize character to index mapping
    let mut c_arr = [0; 26];
    c_arr[(b'F' - b'A') as usize] = 0;
    c_arr[(b'W' - b'A') as usize] = 1;
    c_arr[(b'E' - b'A') as usize] = 2;

    // Initialize result matrix
    let mut res = [[0; 3]; 3];
    res[0][2] = 1;
    res[0][1] = -1;
    res[2][0] = -1;
    res[1][0] = 1;
    res[2][1] = 1;
    res[1][2] = -1;

    // DP arrays: prev_dp holds the previous state, curr_dp is being built
    let mut prev_dp = [[0; 2001]; 3];
    let s_chars: Vec<char> = s.chars().collect();

    // Initialize for the first character
    let first_char = s_chars[0];
    let c = c_arr[(first_char as u8 - b'A') as usize];
    for j in 0..3 {
        let score = res[j][c];
        let idx = 1000 + score;
        prev_dp[j][idx as usize] = 1;
    }

    let mut curr_dp = [[0; 2001]; 3];

    // Process remaining characters
    for &ch in s_chars.iter().skip(1) {
        curr_dp = [[0; 2001]; 3];
        let c = c_arr[(ch as u8 - b'A') as usize];

        for j in 0..3 {
            let score = res[j][c];
            for k in 0..=2000 {
                for j1 in 0..3 {
                    if j1 != j {
                        let prev_k = k as i32 - score;
                        if prev_k >= 0 && prev_k <= 2000 {
                            curr_dp[j][k] = (curr_dp[j][k] + prev_dp[j1][prev_k as usize]) % MAX_SIZE;
                        }
                    }
                }
            }
        }

        prev_dp = curr_dp;
    }

    // Sum all valid sequences with score > 1000
    let mut ans = 0;
    for j in 0..3 {
        for k in 1001..=2000 {
            ans = (ans + prev_dp[j][k]) % MAX_SIZE;
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _char_size: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim();

    println!("{}", count_winning_sequences(s));
}