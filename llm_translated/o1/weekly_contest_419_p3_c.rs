// Problem: Weekly Contest 419 Problem 3 (Translated from C to Rust)

use std::io::{self, BufRead};

/// Counts the number of winning sequences given a string of moves.
/// This preserves the exact logic from the original C code.
fn count_winning_sequences(s: &str) -> i32 {
    // The C code uses 1e9+7, which is 1000000007
    let max_size = 1_000_000_007i32;

    // dp array in C was: int dp[2][3][2001] = {0};
    // We'll store it similarly in Rust as [[[i32; 2001]; 3]; 2].
    // dp[i % 2][j][k] will hold the DP states.
    let mut dp = [[[0i32; 2001]; 3]; 2];

    // res array in C was: int res[3][3] = {0};
    // We'll fill it exactly as in the C code.
    let mut res = [[0i32; 3]; 3];

    // c_arr array in C was: int c_arr[26] = {0};
    // This maps 'F' -> 0, 'W' -> 1, 'E' -> 2. All other letters remain 0 by default.
    let mut c_arr = [0i32; 26];

    // Convert the input string into a byte slice for easy indexing by [i].
    let s_bytes = s.as_bytes();
    let s_len = s_bytes.len();

    // The final answer accumulator
    let mut ans = 0i32;

    // Set up the c_arr mapping exactly like in the C code
    c_arr[(b'F' - b'A') as usize] = 0;
    c_arr[(b'W' - b'A') as usize] = 1;
    c_arr[(b'E' - b'A') as usize] = 2;

    // Set up the res matrix exactly like in the C code
    // res[0][2] = 1;  res[0][1] = -1;
    // res[2][0] = -1; res[1][0] = 1;
    // res[2][1] = 1;  res[1][2] = -1;
    res[0][2] = 1;
    res[0][1] = -1;
    res[2][0] = -1;
    res[1][0] = 1;
    res[2][1] = 1;
    res[1][2] = -1;

    // We use 1000 as the offset in the dp array so we can store negative indices
    // (scores of -1, 0, or 1) around the midpoint of the dp dimension (0..2000).
    let offset = 1000i32;

    // Base case initialization: for the first character in s, set dp[0][i][offset + score] = 1
    // for each possible "previous" move i = 0..2.
    if s_len > 0 {
        let first_c = c_arr[(s_bytes[0] - b'A') as usize] as usize;
        for i in 0..=2 {
            let score = res[i][first_c];
            dp[0][i][(offset + score) as usize] = 1;
        }
    }

    // Fill dp for each subsequent character in s
    for i in 1..s_len {
        // Reset the dp slice for i % 2 before using it
        dp[i % 2] = [[0; 2001]; 3];

        let c = c_arr[(s_bytes[i] - b'A') as usize] as i32;
        for j in 0..=2 {
            let score = res[j][c as usize];
            for k in 0..=2000 {
                for j1 in 0..=2 {
                    if j1 != j {
                        let prev_idx = (k as i32) - score;
                        if prev_idx >= 0 && prev_idx <= 2000 {
                            dp[i % 2][j][k] = (dp[i % 2][j][k]
                                + dp[(i - 1) % 2][j1][prev_idx as usize])
                                % max_size;
                        }
                    }
                }
            }
        }
    }

    // Sum up all dp states that correspond to a net score > 0
    // i.e., indices from 1001..2000 in dp[(s_len-1) % 2]
    if s_len > 0 {
        for i in 0..=2 {
            for j in 1001..=2000 {
                ans = (ans + dp[(s_len - 1) % 2][i][j]) % max_size;
            }
        }
    }

    ans
}

fn main() -> io::Result<()> {
    // We match the exact I/O logic of the original C code:
    // 1) Read an integer (charSize)
    // 2) Allocate memory (not needed in Rust, but we still intentionally read it)
    // 3) Read a string
    // 4) Print the result of count_winning_sequences
    // 5) No other output or prompts

    // Read from stdin line by line
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line for the integer charSize (though we do not actually need to use it to allocate)
    let char_size_line = match lines.next() {
        Some(line) => line?,
        None => return Ok(()),
    };
    let _char_size: usize = match char_size_line.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            // If parsing fails, we mimic the C code's behavior (which doesn't handle errors explicitly).
            // We'll just return 0 or do nothing. But let's just end gracefully.
            return Ok(());
        }
    };

    // Read the second line for the string s
    // (The C code does scanf("%s", s), which reads a whitespace-delimited token.)
    let s_line = match lines.next() {
        Some(line) => line?,
        None => return Ok(()),
    };
    let s = s_line.trim().to_owned();

    let ans = count_winning_sequences(&s);

    // Print the result (same as printf("%d\n", ...))
    println!("{}", ans);

    Ok(())
}