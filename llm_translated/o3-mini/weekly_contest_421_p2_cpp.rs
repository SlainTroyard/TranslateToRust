use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

fn length_after_transformations(s: &str, t: i32) -> i32 {
    let mut cnt = [0_i64; 26];
    
    // Count frequency of each character
    for ch in s.chars() {
        if ch >= 'a' && ch <= 'z' {
            cnt[(ch as u8 - b'a') as usize] += 1;
        }
    }

    // Perform transformations t times
    for _ in 0..t {
        let mut nxt = [0_i64; 26];
        
        // For characters 'a' to 'y': shift count right by one index
        for j in 0..25 {
            nxt[j + 1] = cnt[j];
        }
        // For 'z': distribute its count to first two positions ('a' and 'b')
        nxt[0] = (nxt[0] + cnt[25]) % MOD;
        nxt[1] = (nxt[1] + cnt[25]) % MOD;
        
        cnt = nxt;
    }
    
    // Compute the final answer as the sum of all counts modulo MOD
    let ans = cnt.iter().fold(0, |acc, &x| (acc + x) % MOD);
    ans as i32
}

fn main() -> io::Result<()> {
    // Use stdin to read input; input may contain multiple lines/values as per the original code.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first non-empty line containing the string s
    let s = loop {
        if let Some(line) = lines.next() {
            let line = line?;
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                break trimmed.to_string();
            }
        } else {
            // If input is not provided, exit with error.
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "No input provided for string s"));
        }
    };

    // Read the next non-empty line containing integer t
    let t: i32 = loop {
        if let Some(line) = lines.next() {
            let line = line?;
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                match trimmed.split_whitespace().next() {
                    Some(token) => match token.parse() {
                        Ok(num) => break num,
                        Err(_) => {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidInput,
                                "Invalid integer input for t",
                            ))
                        }
                    },
                    None => continue,
                }
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "No input provided for integer t"));
        }
    };

    // Get transformation result and print it
    let result = length_after_transformations(&s, t);
    println!("{}", result);
    Ok(())
}