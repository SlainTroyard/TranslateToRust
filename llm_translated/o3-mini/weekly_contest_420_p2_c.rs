use std::io::{self, BufRead};

fn number_of_substrings(s: &str, k: i32) -> i32 {
    // We'll work with bytes since we know s contains only lowercase letters.
    let bytes = s.as_bytes();
    let s_l = bytes.len();
    let mut hash_arr = [0; 26];
    let mut left = 0;
    let mut right = 0;
    let mut res = 0;
    
    // Loop until either pointer passes the end of the string
    while left < s_l && right < s_l {
        // Increase count for the character at position 'right'
        let idx = (bytes[right] - b'a') as usize;
        hash_arr[idx] += 1;
        
        // Check if the character count reaches k
        if hash_arr[idx] == k {
            // Every substring ending from right to end is valid
            res += (s_l - right) as i32;
            
            // Try to move left pointer while adjusting counts
            while left <= right {
                let l_idx = (bytes[left] - b'a') as usize;
                hash_arr[l_idx] -= 1;
                left += 1;
                
                // If the decreased count is not equal to k-1, we can continue to add substrings.
                if hash_arr[l_idx] != (k - 1) {
                    res += (s_l - right) as i32;
                } else {
                    // Break out once a character falls below k from its previous exact k.
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
    // Set up a buffered reader for standard input
    let stdin = io::stdin();
    let mut input = String::new();
    
    // Read entire input from STDIN.
    // This works even if the string and integer are provided on multiple lines.
    if let Err(e) = stdin.lock().read_to_string(&mut input) {
        eprintln!("Error reading input: {}", e);
        return;
    }
    
    // Split input by whitespace.
    let mut iter = input.split_whitespace();
    
    // Get the string s; if missing, print an error and exit.
    let s = if let Some(token) = iter.next() {
        token
    } else {
        eprintln!("Error: Expected a string input.");
        return;
    };
    
    // Get the integer k; if missing, print an error and exit.
    let k = if let Some(token) = iter.next() {
        token.parse::<i32>().unwrap_or_else(|e| {
            eprintln!("Error parsing integer k: {}", e);
            std::process::exit(1);
        })
    } else {
        eprintln!("Error: Expected an integer input.");
        return;
    };
    
    // Compute the result using the algorithm.
    let result = number_of_substrings(s, k);
    
    // Print the result exactly as the original code does (with a newline).
    println!("{}", result);
}