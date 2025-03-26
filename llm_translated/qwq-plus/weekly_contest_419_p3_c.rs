use std::io;

const MOD: u32 = 1_000_000_007;

fn count_winning_sequences(s: &str) -> i32 {
    let s_len = s.len();
    let mut res = [[0i32; 3]; 3];
    res[0][2] = 1;
    res[0][1] = -1;
    res[2][0] = -1;
    res[1][0] = 1;
    res[2][1] = 1;
    res[1][2] = -1;

    let mut c_arr = [0i32; 26];
    c_arr['F' as usize - 'A' as usize] = 0;
    c_arr['W' as usize - 'A' as usize] = 1;
    c_arr['E' as usize - 'A' as usize] = 2;

    let s_chars: Vec<char> = s.chars().collect();
    let mut dp = [[[0u32; 2001]; 3]; 2];

    // Initialize first character
    let first_char = s_chars[0];
    let c = c_arr[first_char as usize - 'A' as usize] as usize;
    for j in 0..3 {
        let score = res[j][c] as i32;
        let k = (1000 + score) as usize;
        dp[0][j][k] = 1;
    }

    // Process remaining characters
    for i in 1..s_len {
        let current = i % 2;
        let previous = (i - 1) % 2;

        // Reset current layer to 0
        for j in 0..3 {
            for k in 0..2001 {
                dp[current][j][k] = 0;
            }
        }

        let current_char = s_chars[i];
        let c = c_arr[current_char as usize - 'A' as usize] as usize;
        for j in 0..3 {
            let score = res[j][c] as i32;
            for k in 0..2001 {
                for j1 in 0..3 {
                    if j1 != j {
                        let prev_k = k as i32 - score;
                        if prev_k >= 0 && prev_k < 2001 {
                            let prev_k = prev_k as usize;
                            dp[current][j][k] = (dp[current][j][k] + dp[previous][j1][prev_k]) % MOD;
                        }
                    }
                }
            }
        }
    }

    // Calculate the answer
    let last = (s_len - 1) % 2;
    let mut ans = 0;
    for i in 0..3 {
        for k in 1001..=2000 {
            ans = (ans + dp[last][i][k]) % MOD;
        }
    }
    ans as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let char_size: usize = input.trim().parse().expect("Invalid integer");
    
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read string");
    let s = s.trim().to_string();
    assert!(s.len() == char_size, "String length does not match specified size");
    
    println!("{}", count_winning_sequences(&s));
}