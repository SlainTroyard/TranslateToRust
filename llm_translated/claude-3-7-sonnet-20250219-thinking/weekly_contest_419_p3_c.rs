use std::io::{self, BufRead};

fn count_winning_sequences(s: &str) -> i32 {
    const MAX_SIZE: i32 = 1_000_000_007;
    let mut dp = [[[0i32; 2001]; 3]; 2];
    let mut res = [[0i32; 3]; 3];
    let mut c_arr = [0i32; 26];
    let s_len = s.len();
    let mut ans = 0;
    
    // Map 'F', 'W', 'E' to indices 0, 1, 2
    c_arr['F' as usize - 'A' as usize] = 0;
    c_arr['W' as usize - 'A' as usize] = 1;
    c_arr['E' as usize - 'A' as usize] = 2;
    
    // Define scoring matrix between states
    res[0][2] = 1;
    res[0][1] = -1;
    res[2][0] = -1;
    res[1][0] = 1;
    res[2][1] = 1;
    res[1][2] = -1;
    
    let s_bytes = s.as_bytes();
    
    // Initialize DP for the first character
    for i in 0..=2 {
        let c = c_arr[(s_bytes[0] - b'A') as usize] as usize;
        let score = res[i][c];
        dp[0][i][1000 + score as usize] = 1;
    }
    
    // Process each character in the string
    for i in 1..s_len {
        // Clear the current dp array for this iteration
        for j in 0..3 {
            for k in 0..2001 {
                dp[i % 2][j][k] = 0;
            }
        }
        
        for j in 0..=2 {
            let c = c_arr[(s_bytes[i] - b'A') as usize] as usize;
            let score = res[j][c];
            
            for k in 0..=2000 {
                for j1 in 0..=2 {
                    if j1 != j {
                        let k_minus_score = k as i32 - score;
                        if k_minus_score >= 0 && k_minus_score <= 2000 {
                            dp[i % 2][j][k] += dp[(i - 1) % 2][j1][k_minus_score as usize];
                            dp[i % 2][j][k] %= MAX_SIZE;
                        }
                    }
                }
            }
        }
    }
    
    // Count all sequences with positive score
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
    
    // Read the size of the string (not used in Rust as strings are dynamic)
    let _char_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the string
    let s = lines.next().unwrap().unwrap();
    
    println!("{}", count_winning_sequences(&s));
}