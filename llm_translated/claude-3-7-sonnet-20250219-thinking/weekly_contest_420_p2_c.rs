use std::io;

fn number_of_substrings(s: &str, k: i32) -> i32 {
    let mut hash_arr = [0; 26];
    let mut left = 0;
    let mut right = 0;
    let s_l = s.len();
    let mut res = 0;
    
    let s_bytes = s.as_bytes();
    
    while left < s_l && right < s_l {
        // Increment count for character at right position
        hash_arr[(s_bytes[right] - b'a') as usize] += 1;
        
        if hash_arr[(s_bytes[right] - b'a') as usize] == k {
            // Found a character that appears exactly k times
            res += (s_l - right) as i32;
            
            while left <= right {
                // Decrement count for character at left position
                let idx = (s_bytes[left] - b'a') as usize;
                hash_arr[idx] -= 1;
                left += 1;
                
                if hash_arr[idx] != k - 1 {
                    // If decremented count is not k-1, we can still form valid substrings
                    res += (s_l - right) as i32;
                } else {
                    // We found a character that now appears k-1 times, so we stop
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
    // Read input string and k value
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    let s = parts[0];
    let k: i32 = parts[1].parse().expect("Failed to parse k");
    
    // Calculate and print the result
    println!("{}", number_of_substrings(s, k));
}