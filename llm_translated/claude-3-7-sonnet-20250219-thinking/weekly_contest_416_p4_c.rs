use std::io::{self, BufRead};

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
    
    // Count the occurrences of characters in word2
    for c in word2.bytes() {
        diff[(c - b'a') as usize] -= 1;
    }
    
    // Count how many characters are needed from word1
    let mut cnt = 0;
    for i in 0..26 {
        if diff[i] < 0 {
            cnt += 1;
        }
    }
    
    let mut res = 0i64;
    let mut l = 0;
    let mut r = 0;
    let len1 = word1.len();
    let word1_bytes = word1.as_bytes();
    
    // Sliding window approach
    while l < len1 {
        // Expand window to the right until all character requirements are met
        while r < len1 && cnt > 0 {
            update(&mut diff, (word1_bytes[r] - b'a') as usize, 1, &mut cnt);
            r += 1;
        }
        
        // If all requirements are met, add valid substrings
        if cnt == 0 {
            res += (len1 - r + 1) as i64;
        }
        
        // Shrink window from left
        update(&mut diff, (word1_bytes[l] - b'a') as usize, -1, &mut cnt);
        l += 1;
    }
    
    res
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the length of word1
    let len1: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read word1
    let word1 = lines.next().unwrap()?;
    
    // Read the length of word2
    let len2: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read word2
    let word2 = lines.next().unwrap()?;
    
    // Calculate and print the result
    println!("{}", valid_substring_count(&word1, &word2));
    
    Ok(())
}