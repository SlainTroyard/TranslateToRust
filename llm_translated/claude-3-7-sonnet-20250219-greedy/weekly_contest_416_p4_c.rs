use std::io::{self, BufRead};

/// Updates the difference array and count of negative differences
/// 
/// # Arguments
/// * `diff` - Array of character frequency differences
/// * `c` - Character index to update
/// * `add` - Value to add to the difference
/// * `cnt` - Count of characters with negative differences
fn update(diff: &mut [i32; 26], c: usize, add: i32, cnt: &mut i32) {
    diff[c] += add;
    if add == 1 && diff[c] == 0 {
        *cnt -= 1;
    } else if add == -1 && diff[c] == -1 {
        *cnt += 1;
    }
}

/// Counts valid substrings where all characters in word2 are covered
/// 
/// # Arguments
/// * `word1` - First word
/// * `word2` - Second word
fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut diff = [0; 26];
    
    // Count characters in word2
    for c in word2.bytes() {
        diff[(c - b'a') as usize] -= 1;
    }

    // Count characters with negative differences
    let mut cnt = 0;
    for i in 0..26 {
        if diff[i] < 0 {
            cnt += 1;
        }
    }
    
    let mut res: i64 = 0;
    let mut l = 0;
    let mut r = 0;
    let word1_bytes = word1.as_bytes();
    let len1 = word1_bytes.len();
    
    while l < len1 {
        // Expand window to the right until all characters in word2 are covered
        while r < len1 && cnt > 0 {
            update(&mut diff, (word1_bytes[r] - b'a') as usize, 1, &mut cnt);
            r += 1;
        }
        
        // If all characters in word2 are covered, add valid substrings
        if cnt == 0 {
            res += (len1 - r + 1) as i64;
        }
        
        // Shrink window from the left
        update(&mut diff, (word1_bytes[l] - b'a') as usize, -1, &mut cnt);
        l += 1;
    }

    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read length of word1 (not used in Rust implementation but read to match C input format)
    let _len1: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read word1
    let word1 = lines.next().unwrap().unwrap();
    
    // Read length of word2 (not used in Rust implementation but read to match C input format)
    let _len2: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read word2
    let word2 = lines.next().unwrap().unwrap();
    
    // Calculate and print result
    println!("{}", valid_substring_count(&word1, &word2));
}