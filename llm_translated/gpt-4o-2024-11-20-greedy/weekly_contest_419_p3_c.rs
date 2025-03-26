use std::io::{self, Read};
use std::cmp::max;

const MAX_SIZE: i32 = 1_000_000_007;

fn count_winning_sequences(s: &str) -> i32 {
    let mut dp = vec![vec![vec![0; 2001]; 3]; 2];
    let mut res = vec![vec![0; 3]; 3];
    let mut c_arr = vec![0; 26];
    let s_len = s.len();
    let mut ans = 0;

    // Map characters to indices
    c_arr[('F' as u8 - 'A' as u8) as usize] = 0;
    c_arr[('W' as u8 - 'A' as u8) as usize] = 1;
    c_arr[('E' as u8 - 'A' as u8) as usize] = 2;

    // Initialize the scoring matrix
    res[0][2] = 1;
    res[0][1] = -1;
    res[2][0] = -1;
    res[1][0] = 1;
    res[2][1] = 1;
    res[1][2] = -1;

    // Initialize dp for the first character
    let c = c_arr[(s.chars().nth(0).unwrap() as u8 - 'A' as u8) as usize];
    for i in 0..=2 {
        let score = res[i][c];
        dp[0][i][1000 + score] = 1;
    }

    // Fill dp table for subsequent characters
    for i in 1..s_len {
        dp[i % 2].iter_mut().for_each(|x| x.fill(0));
        let c = c_arr[(s.chars().nth(i).unwrap() as u8 - 'A' as u8) as usize];
        for j in 0..=2 {
            let score = res[j][c];
            for k in 0..=2000 {
                for j1 in 0..=2 {
                    if j1 != j {
                        if k >= score && k - score <= 2000 {
                            dp[i % 2][j][k] += dp[(i - 1) % 2][j1][k - score];
                            dp[i % 2][j][k] %= MAX_SIZE;
                        }
                    }
                }
            }
        }
    }

    // Calculate the final answer
    for i in 0..=2 {
        for j in 1001..=2000 {
            ans += dp[(s_len - 1) % 2][i][j];
            ans %= MAX_SIZE;
        }
    }

    ans
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    // Parse the size of the string (not used directly in Rust)
    let _char_size: usize = lines.next().unwrap().parse().unwrap();
    
    // Parse the string
    let s = lines.next().unwrap();

    // Compute and print the result
    let result = count_winning_sequences(s);
    println!("{}", result);
}