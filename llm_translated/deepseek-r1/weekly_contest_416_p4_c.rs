use std::io::{self, BufRead};

fn update(diff: &mut [i32; 26], c: usize, add: i32, cnt: &mut i32) {
    diff[c] += add;
    if add == 1 {
        if diff[c] == 0 {
            *cnt -= 1;
        }
    } else if add == -1 {
        if diff[c] == -1 {
            *cnt += 1;
        }
    }
}

fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut diff = [0; 26];
    
    // Initialize diff array with word2's character counts (negative)
    for c in word2.chars() {
        let idx = (c as u8 - b'a') as usize;
        diff[idx] -= 1;
    }
    
    // Calculate initial count of characters not satisfied (diff < 0)
    let mut cnt = diff.iter().filter(|&&x| x < 0).count() as i32;
    let mut res = 0i64;
    let len1 = word1.len();
    let word1_bytes = word1.as_bytes();
    let (mut l, mut r) = (0, 0);
    
    while l < len1 {
        // Expand window to the right until all characters are satisfied
        while r < len1 && cnt > 0 {
            let c = (word1_bytes[r] - b'a') as usize;
            update(&mut diff, c, 1, &mut cnt);
            r += 1;
        }
        
        // If all characters are satisfied, add valid substrings count
        if cnt == 0 {
            res += (len1 - r + 1) as i64;
        }
        
        // Move left pointer and update diff
        let c = (word1_bytes[l] - b'a') as usize;
        update(&mut diff, c, -1, &mut cnt);
        l += 1;
    }
    
    res
}

fn main() {
    let stdin = io::stdin();
    let mut tokens = stdin.lock().split_whitespace();
    
    // Read input exactly like original C code (ignoring length parameters)
    let _len1: usize = tokens.next().unwrap().unwrap().parse().unwrap();
    let word1 = tokens.next().unwrap().unwrap();
    let _len2: usize = tokens.next().unwrap().unwrap().parse().unwrap();
    let word2 = tokens.next().unwrap().unwrap();
    
    println!("{}", valid_substring_count(&word1, &word2));
}