use std::io::{self, Write};

const MAX_SIZE: i32 = 1_000_000_007;

fn count_winning_sequences(s: &str) -> i32 {
    let mut dp = [[[0; 2001]; 3]; 2];
    let res = [[0, -1, 1], [1, 0, -1], [-1, 1, 0]];
    let c_arr = [0, 1, 2]; // F -> 0, W -> 1, E -> 2
    let s_len = s.len();
    let mut ans = 0;

    // Initialize dp[0][i][1000 + score] = 1 for each i
    for i in 0..=2 {
        let c = c_arr[(s.chars().nth(0).unwrap() as u8 - b'A') as usize];
        let score = res[i][c];
        dp[0][i][(1000 + score) as usize] = 1;
    }

    // Fill dp table
    for i in 1..s_len {
        let current = i % 2;
        let previous = (i - 1) % 2;
        dp[current] = [[0; 2001]; 3]; // Reset current dp layer

        for j in 0..=2 {
            let c = c_arr[(s.chars().nth(i).unwrap() as u8 - b'A') as usize];
            let score = res[j][c];

            for k in 0..=2000 {
                for j1 in 0..=2 {
                    if j1 != j {
                        let prev_k = k as i32 - score;
                        if prev_k >= 0 && prev_k <= 2000 {
                            dp[current][j][k] = (dp[current][j][k] + dp[previous][j1][prev_k as usize]) % MAX_SIZE;
                        }
                    }
                }
            }
        }
    }

    // Calculate the final answer
    for i in 0..=2 {
        for j in 1001..=2000 {
            ans = (ans + dp[(s_len - 1) % 2][i][j]) % MAX_SIZE;
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let char_size: usize = input.trim().parse().expect("Invalid input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim();

    let result = count_winning_sequences(s);
    println!("{}", result);
}