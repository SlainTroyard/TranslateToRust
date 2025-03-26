use std::io::{self, BufRead};

/// Counts the number of substrings where a specific character appears exactly k times
///
/// # Arguments
/// * `s` - The input string
/// * `k` - The target frequency of a character
///
/// # Returns
/// The number of valid substrings
fn number_of_substrings(s: &str, k: i32) -> i32 {
    let s_chars: Vec<char> = s.chars().collect();
    let s_l = s_chars.len();
    let mut hash_arr = [0; 26];
    let mut left = 0;
    let mut right = 0;
    let mut res = 0;
    
    while left < s_l && right < s_l {
        // Increment count for current character
        hash_arr[(s_chars[right] as u8 - b'a') as usize] += 1;
        
        if hash_arr[(s_chars[right] as u8 - b'a') as usize] == k {
            // Found a substring where the current character appears exactly k times
            res += (s_l - right) as i32;
            
            while left <= right {
                let char_idx = (s_chars[left] as u8 - b'a') as usize;
                hash_arr[char_idx] -= 1;
                left += 1;
                
                if hash_arr[char_idx] != k - 1 {
                    // Keep counting valid substrings
                    res += (s_l - right) as i32;
                } else {
                    break;
                }
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
    
    // Read input line containing space-separated string and integer
    let input_line = lines.next().unwrap().unwrap();
    let mut parts = input_line.split_whitespace();
    
    let s = parts.next().unwrap();
    let k = parts.next().unwrap().parse::<i32>().unwrap();
    
    // Call the function and print the result
    println!("{}", number_of_substrings(s, k));
}