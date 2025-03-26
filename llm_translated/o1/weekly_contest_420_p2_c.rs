use std::io::{self, BufRead};

/// Calculates the number of special substrings of `s` where
/// each character in a sliding window appears exactly k times.
fn number_of_substrings(s: &str, k: i32) -> i32 {
    let mut hash_arr = [0; 26]; // frequency array for 'a'..'z'
    let (mut left, mut right) = (0_usize, 0_usize);
    let s_len = s.len();
    let mut result = 0_i32;

    // Process the string with two pointers
    while left < s_len && right < s_len {
        // Increase count for the current right character
        let idx_right = (s.as_bytes()[right] - b'a') as usize;
        hash_arr[idx_right] += 1;

        // If we've just reached k occurrences of this character
        if hash_arr[idx_right] == k {
            // All substrings starting here until the end qualify
            result += (s_len - right) as i32;

            // Move left pointer until the frequency is no longer k
            while left <= right {
                let idx_left = (s.as_bytes()[left] - b'a') as usize;
                hash_arr[idx_left] -= 1;
                left += 1;

                // If removing this character breaks the k occurrences,
                // add more substrings until we fix it
                if hash_arr[idx_left] != k - 1 {
                    result += (s_len - right) as i32;
                } else {
                    // Once we're back to k-1 for that character, stop
                    break;
                }
            }

            right += 1;
        } else {
            // Otherwise, just move the right pointer forward
            right += 1;
        }
    }

    result
}

fn main() -> io::Result<()> {
    // We need to replicate scanf("%s %d", s, &k) behavior exactly,
    // which may read them from one line or multiple lines.
    // We'll read tokens from stdin until we have at least 2.
    let stdin = io::stdin();
    let mut tokens = Vec::new();

    // Collect tokens across lines until we have 2: one for `s` and one for `k`
    for line in stdin.lock().lines() {
        let line = line?;
        let mut parts = line.trim().split_whitespace();
        while let Some(part) = parts.next() {
            tokens.push(part.to_string());
            if tokens.len() == 2 {
                break;
            }
        }
        if tokens.len() == 2 {
            break;
        }
    }

    // The first token is the string `s`, the second is the integer `k`
    let s = &tokens[0];
    let k: i32 = tokens[1].parse().expect("Failed to parse k as an integer");

    // Compute the result and print
    let res = number_of_substrings(s, k);
    println!("{}", res);

    Ok(())
}