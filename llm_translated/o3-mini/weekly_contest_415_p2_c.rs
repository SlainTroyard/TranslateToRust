use std::cmp;
use std::io::{self, BufRead};
use std::num::ParseIntError;

/// The algorithm: This function mimics the C version of maxScore.
/// It creates a DP table where dp[i][j] represents the maximum score using first
/// i elements from array b and first j elements from array a.
/// It then returns dp[b_size][4].
fn max_score(a: &[i32], b: &[i32]) -> i64 {
    let n = b.len();
    // Create dp table with (n + 1) rows and 5 columns, fill with i64::MIN.
    // The C code used INT_MIN, here we use i64::MIN for safety.
    let mut dp = vec![vec![i64::MIN; 5]; n + 1];
    dp[0][0] = 0;

    // Iterate through b's elements, similar to the C double for-loop.
    // i corresponds to b's index+1. j goes from 0 to 4, representing how many
    // elements from a have been used.
    for i in 1..=n {
        for j in 0..=4 {
            // option: not using b[i-1], so carry forward the value from dp[i-1][j]
            dp[i][j] = dp[i - 1][j];
            // if we have used at least one a element,
            // check if including this pairing gives a higher score.
            if j > 0 {
                dp[i][j] = cmp::max(
                    dp[i][j],
                    dp[i - 1][j - 1] + (a[j - 1] as i64) * (b[i - 1] as i64),
                );
            }
        }
    }
    // The maximum score using all elements in b and 4 pairs from a is at dp[n][4].
    dp[n][4]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use locked stdin for efficient reading.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First, read the line that contains aSize and bSize.
    // The original C code uses scanf("%d %d", ...) which may appear in one or more lines.
    // Here, we join all lines and split whitespace.
    // For more robust error handling, we collect tokens iteratively.
    let mut tokens: Vec<String> = Vec::new();
    while let Some(line) = lines.next() {
        let line = line?;
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }
        tokens.extend(line.split_whitespace().map(String::from));
        // We proceed until we have at least 2 tokens for aSize and bSize.
        if tokens.len() >= 2 {
            break;
        }
    }
    
    // Ensure we have at least two tokens for aSize and bSize.
    if tokens.len() < 2 {
        return Err("Expected at least two integers for aSize and bSize".into());
    }
    
    // Parse aSize and bSize.
    let a_size: usize = tokens[0].parse()?;
    let b_size: usize = tokens[1].parse()?;

    // We need to gather a_size integers for array a.
    let mut a: Vec<i32> = Vec::with_capacity(a_size);
    while a.len() < a_size {
        if tokens.len() > 2 {
            // We already have some numbers in tokens after the first two.
            // Pop tokens until a has a_size numbers.
            let start_index = 2 + a.len();
            if start_index < tokens.len() {
                // Use the token to parse an integer
                let num: i32 = tokens[start_index].parse()?;
                a.push(num);
            } else {
                // if out, break to next reading from lines.
                break;
            }
        } else {
            break;
        }
    }
    
    // If still not enough tokens, continue reading further lines.
    while a.len() < a_size {
        if let Some(line) = lines.next() {
            let line = line?;
            for token in line.split_whitespace() {
                if a.len() < a_size {
                    a.push(token.parse()?);
                } else {
                    break;
                }
            }
        } else {
            break;
        }
    }
    
    // Similarly, gather b_size integers for array b.
    let mut b: Vec<i32> = Vec::with_capacity(b_size);
    // Try to use remaining tokens first.
    // Calculate how many tokens we have already used: 2 for sizes and a_size for array a.
    let used_tokens = 2 + a_size;
    if tokens.len() > used_tokens {
        for token in tokens.iter().skip(used_tokens) {
            if b.len() < b_size {
                b.push(token.parse()?);
            } else {
                break;
            }
        }
    }

    // If not enough, keep reading further lines.
    while b.len() < b_size {
        if let Some(line) = lines.next() {
            let line = line?;
            for token in line.split_whitespace() {
                if b.len() < b_size {
                    b.push(token.parse()?);
                } else {
                    break;
                }
            }
        } else {
            break;
        }
    }
    
    // Check that we have collected the right number of elements.
    if a.len() != a_size {
        return Err(format!("Expected {} integers for array a, found {}", a_size, a.len()).into());
    }
    if b.len() != b_size {
        return Err(format!("Expected {} integers for array b, found {}", b_size, b.len()).into());
    }
    
    // Compute the maximum score using our function.
    let result = max_score(&a, &b);
    
    // Print the result. The original C code prints a long long with "%lld\n"
    // So we do the same (output with newline).
    println!("{}", result);
    
    Ok(())
}