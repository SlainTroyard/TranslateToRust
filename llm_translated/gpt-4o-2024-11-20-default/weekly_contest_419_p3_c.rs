use std::io::{self, Read};

const MAX_SIZE: i32 = 1_000_000_007;

fn count_winning_sequences(s: &str) -> i32 {
    let mut dp = vec![vec![vec![0; 2001]; 3]; 2];
    let mut res = [[0; 3]; 3];
    let mut c_arr = [0; 26];
    let s_bytes = s.as_bytes();
    let s_len = s.len();
    let mut ans = 0;

    // Initialize the scoring rules
    c_arr[(b'F' - b'A') as usize] = 0;
    c_arr[(b'W' - b'A') as usize] = 1;
    c_arr[(b'E' - b'A') as usize] = 2;
    res[0][2] = 1;
    res[0][1] = -1;
    res[2][0] = -1;
    res[1][0] = 1;
    res[2][1] = 1;
    res[1][2] = -1;

    // Initialize dp for the first character
    for i in 0..=2 {
        let c = c_arr[(s_bytes[0] - b'A') as usize] as usize;
        let score = res[i][c];
        dp[0][i][1000 + score as usize] = 1;
    }

    // DP logic for subsequent characters
    for i in 1..s_len {
        dp[i % 2].iter_mut().for_each(|row| row.fill(0)); // Clear current dp

        for j in 0..=2 {
            let c = c_arr[(s_bytes[i] - b'A') as usize] as usize;
            let score = res[j][c];
            for k in 0..=2000 {
                for j1 in 0..=2 {
                    if j1 != j {
                        let target = k as i32 - score;
                        if target >= 0 && target <= 2000 {
                            dp[i % 2][j][k] = (dp[i % 2][j][k]
                                + dp[(i - 1) % 2][j1][target as usize])
                                % MAX_SIZE;
                        }
                    }
                }
            }
        }
    }

    // Count winning sequences (scores > 0)
    for i in 0..=2 {
        for j in 1001..=2000 {
            ans = (ans + dp[(s_len - 1) % 2][i][j]) % MAX_SIZE;
        }
    }

    ans
}

fn main() {
    // Read stdin for inputs
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut lines = input.lines();

    let char_size: usize = lines
        .next()
        .expect("Missing input")
        .trim()
        .parse()
        .expect("Failed to parse char size");

    let s = lines.next().expect("Missing string input").trim();

    if s.len() != char_size {
        panic!("Input string length does not match the specified char size");
    }

    let result = count_winning_sequences(s);
    println!("{}", result);
}