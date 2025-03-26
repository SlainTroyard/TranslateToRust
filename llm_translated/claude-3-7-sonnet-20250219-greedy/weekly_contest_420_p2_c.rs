use std::io::{self, BufRead};

fn number_of_substrings(s: &str, k: i32) -> i32 {
    let s_bytes = s.as_bytes();
    let s_l = s_bytes.len();
    let mut hash_arr = [0; 26];
    let mut left = 0;
    let mut right = 0;
    let mut res = 0;
    
    while left < s_l && right < s_l {
        // Increment the count for the current character
        hash_arr[(s_bytes[right] - b'a') as usize] += 1;
        
        // If we found a character that appears exactly k times
        if hash_arr[(s_bytes[right] - b'a') as usize] == k {
            // Add all possible substrings ending at or after right
            res += (s_l - right) as i32;
            
            // Try to shrink the window from the left
            while left <= right {
                // Decrement the count for the left character
                let left_char_idx = (s_bytes[left] - b'a') as usize;
                hash_arr[left_char_idx] -= 1;
                left += 1;
                
                // If removing this character breaks our condition, stop
                if hash_arr[left_char_idx] == k - 1 {
                    break;
                }
                
                // Otherwise, we can still form valid substrings
                res += (s_l - right) as i32;
            }
            right += 1;
        } else {
            right += 1;
        }
    }
    
    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read input line containing string and k
    if let Some(Ok(line)) = lines.next() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let s = parts[0];
            if let Ok(k) = parts[1].parse::<i32>() {
                println!("{}", number_of_substrings(s, k));
            }
        }
    }
}