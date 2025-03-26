use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input line and split into components
    let input_line = lines.next().expect("Failed to read line").expect("Failed to read line");
    let mut parts = input_line.split_whitespace();
    
    let s = parts.next().expect("No string provided");
    let k: i32 = parts.next().and_then(|p| p.parse().ok()).expect("Invalid k value");
    
    // Calculate and print the result
    println!("{}", number_of_substrings(s, k));
}

fn number_of_substrings(s: &str, k: i32) -> i32 {
    let s_bytes = s.as_bytes();
    let s_len = s_bytes.len();
    let mut hash_arr = [0; 26];
    let (mut left, mut right, mut res) = (0, 0, 0);

    while left < s_len && right < s_len {
        let idx = (s_bytes[right] - b'a') as usize;
        hash_arr[idx] += 1;

        // Check if current character count reaches k
        if hash_arr[idx] == k {
            res += (s_len - right) as i32;

            // Shrink window from left to find minimal valid substrings
            while left <= right {
                let left_idx = (s_bytes[left] - b'a') as usize;
                hash_arr[left_idx] -= 1;
                left += 1;

                // Continue counting if removed character didn't break the k condition
                if hash_arr[left_idx] != k - 1 {
                    res += (s_len - right) as i32;
                } else {
                    break;
                }
            }
        }

        right += 1;
    }

    res
}