use std::io::{self, BufRead};

/// Translates the logic of the C++ solution to Rust.
/// This function calculates the number of valid substrings of `word1`
/// that contain all characters from `word2`.
fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    // diff[i] stores how many more or fewer of character i ('a' + i) are needed
    // to balance word2. For instance, if for 'b' we have 2 more occurrences in
    // word1 than are needed to cover word2 at some point, diff['b'] would be > 0.
    // If we are missing 'c', diff['c'] would be negative.
    let mut diff = vec![0_i32; 26];

    // Decrement counts in `diff` for each character in word2,
    // indicating that we need these characters.
    for c in word2.chars() {
        diff[(c as u8 - b'a') as usize] -= 1;
    }

    // cnt tracks how many distinct characters are still "under-represented"
    // (i.e. diff[c] < 0).
    let mut cnt = diff.iter().filter(|&&val| val < 0).count() as i32;
    let mut res = 0_i64;

    // The update function adjusts diff[c] and updates cnt accordingly.
    // If add == 1 and diff[c] changes from -1 to 0, it means we've
    // just covered one previously under-represented character -> cnt--
    // If add == -1 and diff[c] changes from 0 to -1, we've just lost
    // coverage for a character -> cnt++
    fn update(diff: &mut [i32], cnt: &mut i32, c: usize, add: i32) {
        diff[c] += add;
        // 表明 diff[c] 由 -1 变为 0
        if add == 1 && diff[c] == 0 {
            *cnt -= 1;
        // 表明 diff[c] 由 0 变为 -1
        } else if add == -1 && diff[c] == -1 {
            *cnt += 1;
        }
    }

    // Sliding window approach:
    // Expand `r` while we still need characters (cnt > 0).
    // Once cnt == 0, all required characters are in [l..r),
    // so every substring starting from l and extending beyond r
    // is valid, contributing (word1.len() - r + 1) substrings.
    // Then remove word1[l] from the window and move on.
    let mut r = 0_usize;
    for l in 0..word1.len() {
        while r < word1.len() && cnt > 0 {
            let c_index = (word1.as_bytes()[r] - b'a') as usize;
            update(&mut diff, &mut cnt, c_index, 1);
            r += 1;
        }
        if cnt == 0 {
            res += (word1.len() - r) as i64 + 1;
        }
        let c_index = (word1.as_bytes()[l] - b'a') as usize;
        update(&mut diff, &mut cnt, c_index, -1);
    }
    res
}

fn main() -> io::Result<()> {
    // Read all tokens from stdin
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        for token in line.split_whitespace() {
            tokens.push(token.to_string());
        }
    }

    // Parse inputs in the exact order: len1, word1, len2, word2
    let mut idx = 0;
    let _len1: usize = tokens[idx].parse().unwrap(); // length of word1
    idx += 1;
    let word1 = &tokens[idx];
    idx += 1;
    let _len2: usize = tokens[idx].parse().unwrap(); // length of word2
    idx += 1;
    let word2 = &tokens[idx];
    idx += 1;

    // Compute and print the result
    let result = valid_substring_count(word1, word2);
    println!("{}", result);

    Ok(())
}