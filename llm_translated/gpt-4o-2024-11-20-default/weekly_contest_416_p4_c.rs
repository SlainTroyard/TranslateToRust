use std::io::{self, Read};
use std::cmp::Ordering;

fn update(diff: &mut [i32; 26], c: usize, add: i32, cnt: &mut i32) {
    diff[c] += add;
    if add == 1 && diff[c] == 0 {
        *cnt -= 1;
    } else if add == -1 && diff[c] == -1 {
        *cnt += 1;
    }
}

fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut diff = [0; 26];
    
    // Process `word2` to initialize `diff` array
    for c in word2.chars() {
        let index = (c as u8 - b'a') as usize;
        diff[index] -= 1;
    }

    // Count initial negative values in `diff`
    let mut cnt = 0;
    for val in diff.iter() {
        if *val < 0 {
            cnt += 1;
        }
    }

    // Sliding window
    let mut res = 0;
    let mut l = 0;
    let mut r = 0;
    let len1 = word1.len();
    let word1_bytes = word1.as_bytes();
    
    while l < len1 {
        while r < len1 && cnt > 0 {
            let index = (word1_bytes[r] - b'a') as usize;
            update(&mut diff, index, 1, &mut cnt);
            r += 1;
        }
        if cnt == 0 {
            res += (len1 - r) as i64 + 1; // Substrings starting at `l` and ending at `r`
        }
        let index = (word1_bytes[l] - b'a') as usize;
        update(&mut diff, index, -1, &mut cnt);
        l += 1;
    }

    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Parse the input
    let mut lines = input.lines();
    
    let len1 = lines.next()
        .ok_or("Missing len1")?
        .trim()
        .parse::<usize>()?;
    
    let word1 = lines.next()
        .ok_or("Missing word1")?;
    
    if word1.len() != len1 {
        return Err(Box::from(format!("Word1 length mismatch: expected {}, got {}", len1, word1.len())));
    }

    let len2 = lines.next()
        .ok_or("Missing len2")?
        .trim()
        .parse::<usize>()?;
    
    let word2 = lines.next()
        .ok_or("Missing word2")?;
    
    if word2.len() != len2 {
        return Err(Box::from(format!("Word2 length mismatch: expected {}, got {}", len2, word2.len())));
    }

    // Compute the result
    let result = valid_substring_count(word1, word2);

    // Output the result
    println!("{}", result);

    Ok(())
}