use std::io::{self, BufRead};

fn count_winning_sequences(s: &str) -> i32 {
    const MAX_SIZE: i32 = 1_000_000_007;
    // dp[i % 2][j][k] represents the number of ways to achieve score k 
    // ending with character type j after considering i characters
    let mut dp = [[[0i32; 2001]; 3]; 2];
    // res[i][j] represents the score change when player i follows player j
    let mut res = [[0i32; 3]; 3];
    // c_arr maps characters to their player indices
    let mut c_arr = [0i32; 26];
    
    let s_bytes = s.as_bytes();
    let s_len = s_bytes.len();
    
    // Initialize character mappings
    c_arr[b'F' as usize - b'A' as usize] = 0;
    c_arr[b'W' as usize - b'A' as usize] = 1;
    c_arr[b'E' as usize - b'A' as usize] = 2;
    
    // Initialize score matrix
    res[0][2] = 1;
    res[0][1] = -1;
    res[2][0] = -1;
    res[1][0] = 1;
    res[2][1] = 1;
    res[1][2] = -1;
    
    // Base case: first character
    for i in 0..=2 {
        let c = c_arr[(s_bytes[0] - b'A') as usize] as usize;
        let score = res[i][c];
        dp[0][i][1000 + score as usize] = 1;
    }
    
    // Dynamic programming iteration
    for i in 1..s_len {
        // Clear current dp layer
        for j in 0..3 {
            for k in 0..=2000 {
                dp[i % 2][j][k] = 0;
            }
        }
        
        for j in 0..=2 {
            let c = c_arr[(s_bytes[i] - b'A') as usize] as usize;
            let score = res[j][c];
            
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
    let mut ans = 0;
    for i in 0..=2 {
        for j in 1001..=2000 {
            ans += dp[(s_len - 1) % 2][i][j];
            ans %= MAX_SIZE;
        }
    }
    
    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read character size
    let char_size: usize = lines.next()
        .expect("Failed to read line")
        .expect("Failed to parse line")
        .trim()
        .parse()
        .expect("Failed to parse char_size");
    
    // Read the string
    let s = lines.next()
        .expect("Failed to read line")
        .expect("Failed to parse line");
    
    // Calculate and print result
    println!("{}", count_winning_sequences(&s));
    
    Ok(())
}