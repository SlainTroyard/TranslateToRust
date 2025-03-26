use std::io::{self, BufRead};

fn count_winning_sequences(s: &str) -> i32 {
    const MAX_SIZE: i32 = 1_000_000_007;
    // dp[i % 2][j][k] represents the number of ways to get score k ending with character j at position i
    let mut dp = [[[0i32; 2001]; 3]; 2];
    // Result matrix for different combinations of characters
    let mut res = [[0i32; 3]; 3];
    // Mapping of characters to indices
    let mut c_arr = [0i32; 26];
    let s_len = s.len();
    let mut ans = 0;
    
    // Map characters to indices
    c_arr['F' as usize - 'A' as usize] = 0;
    c_arr['W' as usize - 'A' as usize] = 1;
    c_arr['E' as usize - 'A' as usize] = 2;
    
    // Initialize result matrix for scoring
    res[0][2] = 1;
    res[0][1] = -1;
    res[2][0] = -1;
    res[1][0] = 1;
    res[2][1] = 1;
    res[1][2] = -1;
    
    // Convert string to bytes for easier character access
    let s_bytes = s.as_bytes();
    
    // Initialize dp for the first character
    for i in 0..=2 {
        let c = c_arr[(s_bytes[0] - b'A') as usize];
        let score = res[i as usize][c as usize];
        dp[0][i as usize][(1000 + score) as usize] = 1;
    }
    
    // Fill dp table
    for i in 1..s_len {
        // Clear the current row of dp
        for j in 0..3 {
            for k in 0..=2000 {
                dp[i % 2][j][k] = 0;
            }
        }
        
        for j in 0..=2 {
            let c = c_arr[(s_bytes[i] - b'A') as usize];
            let score = res[j][c as usize];
            
            for k in 0..=2000 {
                for j1 in 0..=2 {
                    if j1 != j {
                        let prev_score = k as i32 - score;
                        if prev_score >= 0 && prev_score <= 2000 {
                            dp[i % 2][j][k] += dp[(i - 1) % 2][j1][prev_score as usize];
                            dp[i % 2][j][k] %= MAX_SIZE;
                        }
                    }
                }
            }
        }
    }
    
    // Count all winning sequences (score > 0)
    for i in 0..=2 {
        for j in 1001..=2000 {
            ans += dp[(s_len - 1) % 2][i][j];
            ans %= MAX_SIZE;
        }
    }
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the character size (though we don't actually need it in Rust)
    let _char_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the string
    let s = lines.next().unwrap().unwrap();
    
    // Calculate and print the result
    println!("{}", count_winning_sequences(&s));
}