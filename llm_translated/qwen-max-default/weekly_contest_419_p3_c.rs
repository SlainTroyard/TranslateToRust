use std::io::{self, BufRead, Write};

const MAX_SIZE: i32 = 1_000_000_007;

fn count_winning_sequences(s: &str) -> i32 {
    let mut dp = [[[0; 2001]; 3]; 2];
    let mut res = [[0; 3]; 3];
    let mut c_arr = [0; 26];

    // Initialize the character mapping
    c_arr['F' as usize - 'A' as usize] = 0;
    c_arr['W' as usize - 'A' as usize] = 1;
    c_arr['E' as usize - 'A' as usize] = 2;

    // Initialize the result matrix
    res[0][2] = 1;
    res[0][1] = -1;
    res[2][0] = -1;
    res[1][0] = 1;
    res[2][1] = 1;
    res[1][2] = -1;

    let s_len = s.len();
    let mut ans = 0;

    // Initialize the first character
    for i in 0..=2 {
        let c = c_arr[s.chars().next().unwrap() as usize - 'A' as usize];
        let score = res[i][c];
        dp[0][i][1000 + score] = 1;
    }

    // Process the rest of the string
    for i in 1..s_len {
        for j in 0..=2 {
            let c = c_arr[s.chars().nth(i).unwrap() as usize - 'A' as usize];
            let score = res[j][c];
            for k in 0..=2000 {
                for j1 in 0..=2 {
                    if j1 != j {
                        if (k - score) >= 0 && (k - score) <= 2000 {
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
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the size of the input string
    stdin.lock().read_line(&mut buffer).unwrap();
    let char_size: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read the input string
    stdin.lock().read_line(&mut buffer).unwrap();
    let s = buffer.trim();

    // Call the function and print the result
    let result = count_winning_sequences(s);
    writeln!(stdout, "{}", result).unwrap();
}