use std::io;

struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut diff = vec![0; 26];
        
        // Initialize diff array with negative counts for characters in word2
        for c in word2.chars() {
            diff[(c as u8 - b'a') as usize] -= 1;
        }
        
        let mut res = 0;
        // Count how many characters have negative counts
        let mut cnt = diff.iter().filter(|&&c| c < 0).count() as i32;
        
        // Closure to update diff and cnt when adding or removing a character
        let mut update = |c: usize, add: i32| {
            diff[c] += add;
            if add == 1 && diff[c] == 0 {
                // This means diff[c] changed from -1 to 0
                cnt -= 1;
            } else if add == -1 && diff[c] == -1 {
                // This means diff[c] changed from 0 to -1
                cnt += 1;
            }
        };
        
        let word1_bytes = word1.as_bytes();
        let mut r = 0;
        
        for l in 0..word1.len() {
            // Extend window to the right until we have balanced all characters
            while r < word1.len() && cnt > 0 {
                update((word1_bytes[r] - b'a') as usize, 1);
                r += 1;
            }
            
            // If all characters are balanced, add valid substrings to result
            if cnt == 0 {
                res += (word1.len() - r + 1) as i64;
            }
            
            // Shrink window from the left
            update((word1_bytes[l] - b'a') as usize, -1);
        }
        
        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _len1: usize = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let word1 = input.trim().to_string();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let _len2: usize = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let word2 = input.trim().to_string();
    
    println!("{}", Solution::valid_substring_count(word1, word2));
}