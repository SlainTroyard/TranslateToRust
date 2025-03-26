use std::error::Error;
use std::io::{self, BufRead, Write};

const MOD: i32 = 1_000_000_007;
const OFFSET: usize = 1000; // index offset: 0 score is stored at index 1000
const MAX_SCORE_INDEX: usize = 2001; // scores from -1000 to 1000

// Map a character to its value: 'F' -> 0, 'W' -> 1, 'E' -> 2
fn map_char(c: char) -> usize {
    match c {
        'F' => 0,
        'W' => 1,
        'E' => 2,
        _ => {
            // This should not happen if input is valid.
            panic!("Unexpected character: {}", c);
        }
    }
}

// Function to count winning sequences based on the given C algorithm logic.
fn count_winning_sequences(s: &str) -> i32 {
    let s_chars: Vec<char> = s.chars().collect();
    let s_len = s_chars.len();

    // res matrix: res[i][c] store points difference when player with role i plays against character with value c.
    // According to the C code:
    // res[0][2] = 1; res[0][1] = -1;
    // res[1][0] = 1; res[1][2] = -1;
    // res[2][0] = -1; res[2][1] = 1;
    let res: [[i32; 3]; 3] = [
        [0, -1, 1],
        [1, 0, -1],
        [-1, 1, 0],
    ];

    // dp is a 3D array: dp[cur_mod][player_index][score_index]
    // We alternate between 0 and 1 for the first dimension.
    let mut dp = [[[0i32; MAX_SCORE_INDEX]; 3]; 2];

    // Initialize dp for the first character.
    let c = map_char(s_chars[0]);
    for i in 0..3 {
        let score = res[i][c]; // score can be -1, 0, or 1
        let index = (OFFSET as i32 + score) as usize;
        dp[0][i][index] = 1;
    }

    // Process subsequent characters.
    for i in 1..s_len {
        let cur_mod = i % 2;
        let prev_mod = (i - 1) % 2;

        // Clear current dp array.
        for j in 0..3 {
            dp[cur_mod][j].iter_mut().for_each(|x| *x = 0);
        }

        let c = map_char(s_chars[i]);
        // For each possible role j for the current character.
        for j in 0..3 {
            // Determine the score contributed by role j playing against character c.
            let score = res[j][c];
            // For every possible cumulative score from previous rounds.
            for k in 0..MAX_SCORE_INDEX {
                // Try all previous roles j1 different from the current role j.
                for j1 in 0..3 {
                    if j1 == j {
                        continue;
                    }
                    // Calculate the previous score index needed.
                    // k (current index) = (previous index) + score, so previous index = k - score.
                    let prev_index = (k as i32) - score;
                    if prev_index >= 0 && (prev_index as usize) < MAX_SCORE_INDEX {
                        dp[cur_mod][j][k] =
                            (dp[cur_mod][j][k] + dp[prev_mod][j1][prev_index as usize]) % MOD;
                    }
                }
            }
        }
    }

    // Sum all dp entries (for any role) where the cumulative score is positive (i.e. index > OFFSET).
    let mut ans = 0;
    let last_mod = (s_len - 1) % 2;
    for i in 0..3 {
        for k in (OFFSET + 1)..MAX_SCORE_INDEX {
            ans = (ans + dp[last_mod][i][k]) % MOD;
        }
    }

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    // Prepare for reading input from stdin.
    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();

    // The C code reads an integer representing the size of the string
    // and then reads the string itself.
    let char_size_line = reader
        .next()
        .ok_or("Missing char size input")??;
    let _char_size: usize = char_size_line
        .trim()
        .parse()
        .map_err(|_| "Failed to parse char size as integer")?;
    
    // Read the next line as the string input.
    let s_line = reader
        .next()
        .ok_or("Missing string input")??;
    let s = s_line.trim();

    // Compute the result using the provided algorithm.
    let result = count_winning_sequences(s);

    // Output the result exactly as in the original code.
    println!("{}", result);

    Ok(())
}