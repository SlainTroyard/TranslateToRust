use std::io::{self, Write};

// Function to update the difference array and the count of non-zero differences
fn update(diff: &mut [i32; 26], c: usize, add: i32, cnt: &mut i32) {
    diff[c] += add;
    if add == 1 && diff[c] == 0 {
        *cnt -= 1;
    } else if add == -1 && diff[c] == -1 {
        *cnt += 1;
    }
}

// Function to count the number of valid substrings
fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut diff = [0; 26];
    
    // Initialize the difference array based on word2
    for c in word2.chars() {
        diff[(c as u8 - b'a') as usize] -= 1;
    }
    
    // Count the number of negative differences
    let mut cnt = diff.iter().filter(|&&x| x < 0).count() as i32;
    
    let mut res = 0;
    let len1 = word1.len();
    let word1_chars: Vec<char> = word1.chars().collect();
    let mut l = 0;
    let mut r = 0;
    
    while l < len1 {
        // Expand the window to the right until all differences are non-negative
        while r < len1 && cnt > 0 {
            update(&mut diff, (word1_chars[r] as u8 - b'a') as usize, 1, &mut cnt);
            r += 1;
        }
        
        // If all differences are non-negative, add the number of valid substrings
        if cnt == 0 {
            res += (len1 - r + 1) as i64;
        }
        
        // Move the left pointer and update the difference array
        update(&mut diff, (word1_chars[l] as u8 - b'a') as usize, -1, &mut cnt);
        l += 1;
    }
    
    res
}

fn main() {
    let mut input = String::new();
    
    // Read the length of word1
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let len1: usize = input.trim().parse().expect("Invalid input");
    
    // Read word1
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let word1 = input.trim().to_string();
    
    // Read the length of word2
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let len2: usize = input.trim().parse().expect("Invalid input");
    
    // Read word2
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let word2 = input.trim().to_string();
    
    // Calculate and print the result
    println!("{}", valid_substring_count(&word1, &word2));
}